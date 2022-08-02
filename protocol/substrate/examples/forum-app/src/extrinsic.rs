use crate::{
    content::*,
    Api,
    Error,
};
use async_recursion::async_recursion;
use codec::{
    Compact,
    Decode,
    Encode,
    Input,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuntimeVersion {
    pub spec_name: String,
    pub impl_name: String,
    pub authoring_version: u32,
    pub spec_version: u32,
    pub impl_version: u32,
    pub transaction_version: u32,
    pub state_version: u8,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UncheckedExtrinsicV4<Call> {
    pub signature: Option<(
        MultiAddress<AccountId32, ()>,
        MultiSignature,
        (Era, Compact<u32>, Compact<u128>),
    )>,
    pub function: Call,
}

#[derive(Decode, Encode, Clone, Eq, PartialEq, Debug)]
pub struct SignedPayload<Call> {
    call: Call,
    extra: (Era, Compact<u32>, Compact<u128>), //(era, nonce, tip)
    additional: (u32, u32, H256, H256), // (spec_version, transaction_version, genesis_hash, mortality_check_hash)
}

/// TODO: This should be hookup to the browser extension
pub async fn sign_call_and_encode<Call>(
    api: &Api,
    call: Call,
    tip: Option<u128>,
) -> Result<String, Error>
where
    Call: Encode + Clone + fmt::Debug,
{
    // we use alice for now, for simplicity
    let signer: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    // TODO: use the signer provider as the signing function
    let signing_function = |payload: &[u8]| signer.sign(payload);

    let multi_signer = MultiSigner::from(signer.public());
    let signer_account = multi_signer.into_account();

    let nonce = get_nonce_for_account(api, &signer_account).await?;
    let signer_address: MultiAddress<AccountId32, ()> = signer_account.into();

    let runtime_version = get_runtime_version(api).await?;
    let genesis_hash = get_genesis_hash(api)
        .await?
        .expect("must have a genesis hash");

    let extra = (Era::Immortal, Compact(nonce), Compact(tip.unwrap_or(0)));

    let raw_payload = SignedPayload {
        call: call.clone(),
        extra: extra.clone(),
        additional: (
            runtime_version.spec_version,
            runtime_version.transaction_version,
            genesis_hash,
            genesis_hash,
        ),
    };

    let signature = raw_payload.using_encoded(signing_function);
    let multi_signature = MultiSignature::from(signature);

    let extrinsic = UncheckedExtrinsicV4 {
        function: call,
        signature: Some((signer_address, multi_signature, extra)),
    };

    let encoded = extrinsic.hex_encode();
    Ok(encoded)
}

pub async fn get_nonce_for_account(
    api: &Api,
    account: &AccountId32,
) -> Result<u32, Error> {
    let args = json!({
        "url": api.url,
        "account": account.to_ss58check(),
    });
    let nonce: u32 = api.invoke_method("getNonceForAccount", args).await?;
    Ok(nonce)
}

pub async fn get_genesis_hash(api: &Api) -> Result<Option<H256>, Error> {
    let args = json!({
        "url": api.url,
    });
    let runtime_version: Option<H256> =
        api.invoke_method("genesisHash", args).await?;
    Ok(runtime_version)
}

pub async fn get_runtime_version(api: &Api) -> Result<RuntimeVersion, Error> {
    let args = json!({
        "url": api.url,
    });
    let runtime_version: RuntimeVersion =
        api.invoke_method("getRuntimeVersion", args).await?;
    Ok(runtime_version)
}

impl<Call> UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    pub fn hex_encode(&self) -> String {
        let mut hex_str = hex::encode(self.encode());
        hex_str.insert_str(0, "0x");
        hex_str
    }
}

const V4: u8 = 4;

impl<Call> Encode for UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    fn encode(&self) -> Vec<u8> {
        encode_with_vec_prefix::<Self, _>(|v| {
            match self.signature.as_ref() {
                Some(s) => {
                    v.push(V4 | 0b1000_0000);
                    s.encode_to(v);
                }
                None => {
                    v.push(V4 & 0b0111_1111);
                }
            }
            self.function.encode_to(v);
        })
    }
}

/// Same function as in primitives::generic. Needed to be copied as it is private there.
fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(
    encoder: F,
) -> Vec<u8> {
    let size = sp_std::mem::size_of::<T>();
    let reserve = match size {
        0..=0b0011_1111 => 1,
        0b0100_0000..=0b0011_1111_1111_1111 => 2,
        _ => 4,
    };
    let mut v = Vec::with_capacity(reserve + size);
    v.resize(reserve, 0);
    encoder(&mut v);

    // need to prefix with the total length to ensure it's binary compatible with
    // Vec<u8>.
    let mut length: Vec<()> = Vec::new();
    length.resize(v.len() - reserve, ());
    length.using_encoded(|s| {
        v.splice(0..reserve, s.iter().cloned());
    });

    v
}

impl<Call> Decode for UncheckedExtrinsicV4<Call>
where
    Call: Decode + Encode,
{
    fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
        // This is a little more complicated than usual since the binary format must be compatible
        // with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
        // will be a prefix of vector length (we don't need
        // to use this).
        let _length_do_not_remove_me_see_above: Vec<()> =
            Decode::decode(input)?;

        let version = input.read_byte()?;

        let is_signed = version & 0b1000_0000 != 0;
        let version = version & 0b0111_1111;
        if version != V4 {
            return Err("Invalid transaction version".into());
        }

        Ok(UncheckedExtrinsicV4 {
            signature: if is_signed {
                Some(Decode::decode(input)?)
            } else {
                None
            },
            function: Decode::decode(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::{
        Pair,
        H256 as Hash,
    };
    use sp_runtime::{
        generic::Era,
        testing::sr25519,
        MultiSignature,
    };

    #[test]
    fn encode_decode_roundtrip_works() {
        let msg = &b"test-message"[..];
        let (pair, _) = sr25519::Pair::generate();
        let signature = pair.sign(&msg);
        let multi_sig = MultiSignature::from(signature);
        let account: AccountId32 = pair.public().into();
        let genesis_hash = H256::from([0u8; 32]);

        let extra = (Era::Immortal, Compact(0), Compact(0));
        let additional = (0, 0, 0, genesis_hash, genesis_hash);
        let xt = UncheckedExtrinsicV4 {
            function: [1, 1],
            signature: Some((account.into(), multi_sig, extra)),
        };
        let xt_enc = xt.encode();
        assert_eq!(xt, Decode::decode(&mut xt_enc.as_slice()).unwrap())
    }
}
