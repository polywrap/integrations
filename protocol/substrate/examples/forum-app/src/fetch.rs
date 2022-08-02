use crate::{
    content::*,
    extrinsic::sign_call_and_encode,
    Api,
    Error,
};
use async_recursion::async_recursion;
use codec::{
    Compact,
    Decode,
    Encode,
};
use frame_support::{
    traits::Get,
    BoundedVec,
};
use sauron::prelude::*;
use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use serde_json::json;
use sp_core::{
    crypto::{
        AccountId32,
        Ss58Codec,
    },
    Pair,
    H256,
};
use sp_keyring::AccountKeyring;
use sp_runtime::{
    generic::Era,
    traits::IdentifyAccount,
    MultiAddress,
    MultiSignature,
    MultiSigner,
};
use std::fmt;

const FORUM_MODULE: &str = "ForumModule";
const ALL_POSTS: &str = "AllPosts";
const ALL_COMMENTS: &str = "AllComments";
const KIDS: &str = "Kids";

pub async fn get_post_list(api: &Api) -> Result<Vec<PostDetail>, Error> {
    let mut all_post = Vec::with_capacity(10);
    log::info!("---->Getting all the post_id...");

    let args = json!({
        "url": api.url,
        "pallet": FORUM_MODULE,
        "storage": ALL_POSTS,
        "count": 20,
        "nextTo": None::<u32>,
    });

    let storage_values: Option<Vec<Vec<u8>>> =
        api.invoke_method("getStorageMapPaged", args).await?;

    log::info!("here..");
    if let Some(storage_values) = storage_values {
        for (i, bytes) in storage_values.into_iter().enumerate() {
            log::info!("At post: {}", i);
            let post: Option<Post> = Post::decode(&mut bytes.as_slice()).ok();
            log::info!("post: {:#?}", post);
            if let Some(post) = post {
                let reply_count = get_reply_count(api, post.post_id).await?;
                let block_hash = get_block_hash(api, post.block_number)
                    .await?
                    .expect("must have a block_hash");
                all_post.push(PostDetail {
                    post,
                    reply_count,
                    comments: vec![],
                    block_hash,
                });
            }
        }
    }
    log::info!("done get_post_list..: {:#?}", all_post);
    all_post.sort_unstable_by_key(|item| item.post.post_id);
    Ok(all_post)
}

///TODO: include the reply count into the Post and Comment storage
/// instead of recursively querying here
pub async fn get_reply_count(api: &Api, post_id: u32) -> Result<usize, Error> {
    let reply_count = get_kids(api, post_id)
        .await?
        .map(|kids| kids.len())
        .unwrap_or(0);
    Ok(reply_count)
}

pub async fn get_post_details(
    api: &Api,
    post_id: u32,
) -> Result<Option<PostDetail>, Error> {
    log::info!("getting the post details of {}", post_id);
    let post = get_post(api, post_id).await?;
    if let Some(post) = post {
        let comment_replies = get_comment_replies(api, post_id, 0).await?;
        let reply_count = get_reply_count(api, post_id).await?;
        let block_hash = get_block_hash(api, post.block_number)
            .await?
            .expect("must have a block_hash");
        Ok(Some(PostDetail {
            post,
            comments: comment_replies,
            reply_count,
            block_hash,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_post(api: &Api, post_id: u32) -> Result<Option<Post>, Error> {
    let args = json!({
        "url": api.url,
        "pallet": FORUM_MODULE,
        "storage": ALL_POSTS,
        "key": post_id.to_string(),
    });
    let post: Option<Vec<u8>> =
        api.invoke_method("getStorageMap", args).await?;
    if let Some(post) = post {
        let post: Option<Post> = Post::decode(&mut post.as_slice()).ok();
        Ok(post)
    } else {
        Ok(None)
    }
}

async fn get_kids(
    api: &Api,
    item_id: u32,
) -> Result<Option<BoundedVec<u32, MaxComments>>, Error> {
    let args = json!({
        "url": api.url,
        "pallet": FORUM_MODULE,
        "storage": KIDS,
        "key": item_id.to_string(),
    });
    let kids: Option<Vec<u8>> =
        api.invoke_method("getStorageMap", args).await?;
    if let Some(kids) = kids {
        let kids: Option<BoundedVec<u32, MaxComments>> =
            Decode::decode(&mut kids.as_slice()).ok();
        Ok(kids)
    } else {
        Ok(None)
    }
}

#[async_recursion(?Send)]
pub async fn get_comment_replies(
    api: &Api,
    item_id: u32,
    current_depth: usize,
) -> Result<Vec<CommentDetail>, Error> {
    let mut comment_details = vec![];
    if current_depth >= crate::COMMENT_DEPTH {
        return Ok(vec![]);
    }
    if let Some(kids) = get_kids(api, item_id).await? {
        log::info!("kids of item_id: {} are: {:?}", item_id, kids);
        for kid in kids {
            log::info!("getting comment: {}", kid);
            let comment = get_comment(api, kid)
                .await?
                .expect("must have a comment entry");

            let kid_comments =
                get_comment_replies(api, kid, current_depth + 1).await?;
            let block_hash = get_block_hash(api, comment.block_number)
                .await?
                .expect("must have a block_hash");
            comment_details.push(CommentDetail {
                comment,
                kids: kid_comments,
                block_hash,
            });
        }
    }
    comment_details.sort_unstable_by_key(|item| item.comment.comment_id);
    Ok(comment_details)
}

pub async fn get_comment_detail(
    api: &Api,
    comment_id: u32,
) -> Result<Option<CommentDetail>, Error> {
    if let Some(comment) = get_comment(api, comment_id).await? {
        let kid_comments = get_comment_replies(api, comment_id, 0).await?;
        let block_hash = get_block_hash(api, comment.block_number)
            .await?
            .expect("must have a block hash");

        Ok(Some(CommentDetail {
            comment,
            kids: kid_comments,
            block_hash,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_comment(
    api: &Api,
    comment_id: u32,
) -> Result<Option<Comment>, Error> {
    let args = json!({
        "url": api.url,
        "pallet": FORUM_MODULE,
        "storage": ALL_COMMENTS,
        "key": comment_id.to_string(),
    });
    let comment: Option<Vec<u8>> =
        api.invoke_method("getStorageMap", args).await?;
    if let Some(comment) = comment {
        let comment: Option<Comment> =
            Decode::decode(&mut comment.as_slice()).ok();
        Ok(comment)
    } else {
        Ok(None)
    }
}

pub async fn get_block_hash(
    api: &Api,
    number: u32,
) -> Result<Option<String>, Error> {
    let args = json!({
        "url": api.url,
        "number": number,
    });
    let block_hash: Option<String> =
        api.invoke_method("blockHash", args).await?;
    Ok(block_hash)
}

pub async fn add_post(api: &Api, post: &str) -> Result<Option<H256>, Error> {
    let bounded_content = BoundedVec::try_from(post.as_bytes().to_vec())
        .or_else(|_e| {
            Err(Error::ContentTooLong(post.len(), MaxContentLength::get()))
        })?;

    let pallet_call =
        pallet_call_index(api, FORUM_MODULE, "post_content").await?;
    let call: ([u8; 2], BoundedVec<u8, MaxContentLength>) =
        (pallet_call, bounded_content);

    let extrinsic = sign_call_and_encode(api, call, None).await?;
    log::info!("added a post..");
    let tx_hash = author_submit_extrinsic(api, extrinsic).await?;

    Ok(tx_hash)
}

pub async fn add_comment(
    api: &Api,
    parent_item: u32,
    comment: &str,
) -> Result<Option<H256>, Error> {
    let bounded_content = BoundedVec::try_from(comment.as_bytes().to_vec())
        .or_else(|_e| {
            Err(Error::ContentTooLong(
                comment.len(),
                MaxContentLength::get(),
            ))
        })?;

    let pallet_call =
        pallet_call_index(api, FORUM_MODULE, "comment_on").await?;
    let call: ([u8; 2], u32, BoundedVec<u8, MaxContentLength>) =
        (pallet_call, parent_item, bounded_content);

    let extrinsic = sign_call_and_encode(api, call, None).await?;
    log::debug!("Added a comment to parent_item: {}", parent_item);
    let tx_hash = author_submit_extrinsic(api, extrinsic).await?;

    Ok(tx_hash)
}

/// send some certain amount to this user
pub async fn send_reward(
    api: &Api,
    to: AccountId32,
    amount: u128,
    tip: Option<u128>,
) -> Result<Option<H256>, Error> {
    let balance_transfer_call_index: [u8; 2] =
        pallet_call_index(api, "Balances", "transfer").await?;

    let dest: MultiAddress<AccountId32, ()> = MultiAddress::Id(to);

    let balance_transfer_call: (
        [u8; 2],
        MultiAddress<AccountId32, ()>,
        Compact<u128>,
    ) = (balance_transfer_call_index, dest, Compact(amount));

    let extrinsic = sign_call_and_encode(api, balance_transfer_call,tip).await?;
    let tx_hash = author_submit_extrinsic(api, extrinsic).await?;
    log::debug!("Sent some coins to with a tx_hash: {:?}", tx_hash);
    Ok(tx_hash)
}

pub async fn author_submit_extrinsic(
    api: &Api,
    extrinsic: String,
) -> Result<Option<H256>, Error> {
    let args = json!({
        "url": api.url,
        "extrinsic": extrinsic,
    });
    let block_hash: Option<H256> =
        api.invoke_method("authorSubmitExtrinsic", args).await?;
    Ok(block_hash)
}

pub async fn pallet_call_index(
    api: &Api,
    pallet: &str,
    call: &str,
) -> Result<[u8; 2], Error> {
    let args = json!({
        "url": api.url,
        "pallet": pallet,
        "call": call,
    });
    let result: Option<Vec<u8>> =
        api.invoke_method("palletCallIndex", args).await?;

    let result = result.expect("must have a result");
    Ok([result[0], result[1]])
}
