#![deny(warnings)]

#[allow(warnings)]
pub mod wrap;
pub use api::Api;
use api::BaseApi;
pub use error::Error;
use num_traits::cast::FromPrimitive;
use polywrap_wasm_rs::BigNumber;
use scale_info::{
    TypeDef,
    TypeDefPrimitive,
};
use sp_core::crypto::{
    AccountId32,
    Ss58Codec,
};
pub use types::metadata::Metadata;
use wrap::imported::*;
pub use wrap::*;

mod api;
mod error;
mod types;
mod utils;

#[macro_export]
macro_rules! debug {
    ($($arg: expr),*) => {
        polywrap_wasm_rs::wrap_debug_log(&format!($($arg,)*));
    }
}

getrandom::register_custom_getrandom!(custom_random_number);

/// TODO: use polywraps random plugin for this
/// [#901](https://github.com/polywrap/monorepo/issues/901)
pub fn custom_random_number(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    for b in buf.iter_mut() {
        *b = 4;
    }
    Ok(())
}

/// return the chain metadata
pub fn chain_get_metadata(arg: ArgsChainGetMetadata) -> Option<ChainMetadata> {
    let metadata = BaseApi::new(&arg.url).fetch_metadata();
    let meta = metadata.ok().flatten().expect("must have a metadata");

    let meta_json =
        serde_json::to_value(meta.metadata).expect("unable to convert to json");
    let pallet_json =
        serde_json::to_value(meta.pallets).expect("unable to convert to json");

    let events = meta.events.into_values().collect::<Vec<_>>();
    let events_json =
        serde_json::to_value(events).expect("unable to convert to json");
    let errors = meta.errors.into_values().collect::<Vec<_>>();
    let errors_json =
        serde_json::to_value(errors).expect("unable to convert to json");

    Some(ChainMetadata {
        metadata: meta_json,
        pallets: pallet_json,
        events: events_json,
        errors: errors_json,
    })
}

/// return the chain
pub fn get_runtime_version(
    arg: ArgsGetRuntimeVersion,
) -> Option<RuntimeVersion> {
    BaseApi::new(&arg.url)
        .fetch_runtime_version()
        .ok()
        .flatten()
        .map(|v| {
            RuntimeVersion {
                spec_name: v.spec_name.to_string(),
                impl_name: v.impl_name.to_string(),
                authoring_version: v.authoring_version,
                spec_version: v.spec_version,
                impl_version: v.impl_version,
                transaction_version: v.transaction_version,
                state_version: v.state_version,
            }
        })
}

/// return the rpc methods
pub fn rpc_methods(arg: ArgsRpcMethods) -> Option<Vec<String>> {
    BaseApi::new(&arg.url).fetch_rpc_methods().ok().flatten()
}

/// return the block has of a block with number
pub fn block_hash(arg: ArgsBlockHash) -> Option<String> {
    let api = BaseApi::new(&arg.url);
    let block_hash = api.fetch_block_hash(arg.number);
    block_hash.ok().flatten().map(|h| format!("{:#x}", h))
}

/// return the genesis_hash
pub fn genesis_hash(arg: ArgsGenesisHash) -> Option<String> {
    let api = BaseApi::new(&arg.url);
    let block_hash = api.fetch_genesis_hash();
    block_hash.ok().flatten().map(|h| format!("{:#x}", h))
}

/// return the Block at number
pub fn chain_get_block(arg: ArgsChainGetBlock) -> Option<BlockOutput> {
    let api = BaseApi::new(&arg.url);
    let block = api.fetch_opaque_block(arg.number);
    block.ok().flatten().map(|block| BlockOutput { block })
}

/// return value of storage from a module and storage name
pub fn get_storage_value(arg: ArgsGetStorageValue) -> Option<Vec<u8>> {
    if let Ok(api) = Api::new(&arg.url) {
        api.fetch_opaque_storage_value(&arg.pallet, &arg.storage)
            .ok()
            .flatten()
    } else {
        None
    }
}

/// return value of the storage map from a module and storage name with a certain key
pub fn get_storage_map(arg: ArgsGetStorageMap) -> Option<Vec<u8>> {
    Api::new(&arg.url)
        .ok()
        .map(|api| {
            let (key_type, _value_type) = api
                .storage_map_type(&arg.pallet, &arg.storage)
                .ok()
                .flatten()
                .expect("must have a type");

            if let TypeDef::Primitive(primitive) = key_type.type_def() {
                //TODO: make this exhaustive to support i8,i16,i32,i64,i128 and u8,u16,u32,u64,u128
                match primitive {
                    TypeDefPrimitive::U32 => {
                        assert!(arg.key.is_number());
                        let key =
                            arg.key.as_u64().expect("must cast to u64") as u32;
                        api.fetch_opaque_storage_map(
                            &arg.pallet,
                            &arg.storage,
                            key,
                        )
                        .ok()
                        .flatten()
                    }
                    TypeDefPrimitive::U64 => {
                        assert!(arg.key.is_number());
                        let key =
                            arg.key.as_u64().expect("must cast to u64") as u64;
                        api.fetch_opaque_storage_map(
                            &arg.pallet,
                            &arg.storage,
                            key,
                        )
                        .ok()
                        .flatten()
                    }
                    TypeDefPrimitive::U128 => {
                        assert!(arg.key.is_number());
                        let key =
                            arg.key.as_u64().expect("must cast to u64") as u128;
                        api.fetch_opaque_storage_map(
                            &arg.pallet,
                            &arg.storage,
                            key,
                        )
                        .ok()
                        .flatten()
                    }
                    _ => unimplemented!(),
                }
            } else {
                //TODO: this should be error for UnSupported key type
                None
            }
        })
        .flatten()
}

pub fn get_storage_map_paged(
    arg: ArgsGetStorageMapPaged,
) -> Option<Vec<Vec<u8>>> {
    Api::new(&arg.url)
        .ok()
        .map(|api| {
            let (key_type, _value_type) = api
                .storage_map_type(&arg.pallet, &arg.storage)
                .ok()
                .flatten()
                .expect("must have a type");

            if let TypeDef::Primitive(primitive) = key_type.type_def() {
                //TODO: make this exhaustive to support i8,i16,i32,i64,i128 and u8,u16,u32,u64,u128
                match primitive {
                    TypeDefPrimitive::U32 => {
                        let next_to = arg
                            .next_to
                            .map(|k| k.as_u64().map(|k| k as u32))
                            .flatten();
                        api.fetch_opaque_storage_map_paged(
                            &arg.pallet,
                            &arg.storage,
                            arg.count,
                            next_to,
                        )
                        .ok()
                        .flatten()
                    }
                    TypeDefPrimitive::U64 => {
                        let next_to = arg
                            .next_to
                            .map(|k| k.as_u64().map(|k| k as u64))
                            .flatten();
                        api.fetch_opaque_storage_map_paged(
                            &arg.pallet,
                            &arg.storage,
                            arg.count,
                            next_to,
                        )
                        .ok()
                        .flatten()
                    }
                    TypeDefPrimitive::U128 => {
                        let next_to = arg
                            .next_to
                            .map(|k| k.as_u64().map(|k| k as u128))
                            .flatten();
                        api.fetch_opaque_storage_map_paged(
                            &arg.pallet,
                            &arg.storage,
                            arg.count,
                            next_to,
                        )
                        .ok()
                        .flatten()
                    }
                    _ => unimplemented!(),
                }
            } else {
                None
            }
        })
        .flatten()
}

/// return the constant value from a pallet
pub fn constant(arg: ArgsConstant) -> Option<Vec<u8>> {
    Api::new(&arg.url)
        .ok()
        .map(|api| api.fetch_constant_opaque_value(&arg.pallet, &arg.name).ok())
        .flatten()
}

pub fn account_info(arg: ArgsAccountInfo) -> Option<AccountInfo> {
    let account_id = AccountId32::from_ss58check(&arg.account)
        .expect("must be a valid ss58check format");
    let account_info: Option<crate::types::account_info::AccountInfo> =
        Api::new(&arg.url)
            .ok()
            .map(|api| api.get_account_info(&account_id).ok().flatten())
            .flatten();

    debug!("account info: {:#?}", account_info);

    if let Some(account_info) = account_info {
        Some(AccountInfo {
            nonce: account_info.nonce,
            consumers: account_info.consumers,
            providers: account_info.providers,
            sufficients: account_info.sufficients,
            data: AccountData {
                free: BigNumber::from_u128(account_info.data.free).unwrap(),
                reserved: BigNumber::from_u128(account_info.data.reserved)
                    .unwrap(),
                misc_frozen: BigNumber::from_u128(
                    account_info.data.misc_frozen,
                )
                .unwrap(),
                fee_frozen: BigNumber::from_u128(account_info.data.fee_frozen)
                    .unwrap(),
            },
        })
    } else {
        None
    }
}

/// return the nonce for this account
pub fn get_nonce_for_account(arg: ArgsGetNonceForAccount) -> Option<u32> {
    let account_id = AccountId32::from_ss58check(&arg.account)
        .expect("must be a valid ss58check format");
    Api::new(&arg.url)
        .ok()
        .map(|api| api.get_nonce_for_account(&account_id).ok())
        .flatten()
}

pub fn pallet_call_index(arg: ArgsPalletCallIndex) -> Option<Vec<u8>> {
    Api::new(&arg.url)
        .ok()
        .map(|api| {
            api.pallet_call_index(&arg.pallet, &arg.call)
                .ok()
                .map(|v| v.to_vec())
        })
        .flatten()
}

pub fn author_submit_extrinsic(
    arg: ArgsAuthorSubmitExtrinsic,
) -> Option<String> {
    BaseApi::new(&arg.url)
        .author_submit_extrinsic(arg.extrinsic)
        .ok()
        .flatten()
        .map(|hash| format!("{:#x}", hash))
}
