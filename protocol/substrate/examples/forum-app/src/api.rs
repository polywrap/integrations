use crate::{
    content::*,
    Error,
};
use async_recursion::async_recursion;
use codec::{
    Compact,
    Decode,
    Encode,
};
use frame_support::BoundedVec;
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

#[wasm_bindgen]
extern "C" {

    #[derive(Debug, Clone)]
    pub type PolywrapClientWrapper;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PolywrapClientWrapper;

    #[wasm_bindgen(method)]
    pub async fn invoke_method(
        this: &PolywrapClientWrapper,
        method: &str,
        args: JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub async fn invoke(this: &PolywrapClientWrapper, args: JsValue)
        -> JsValue;
}

#[derive(Clone)]
pub struct Api {
    client: PolywrapClientWrapper,
    pub url: String,
}

impl Api {
    pub async fn new(url: &str) -> Result<Self, Error> {
        Ok(Self {
            client: PolywrapClientWrapper::new(),
            url: url.to_string(),
        })
    }

    pub async fn invoke_method<D: DeserializeOwned>(
        &self,
        method: &str,
        args: serde_json::Value,
    ) -> Result<D, Error> {
        let args = JsValue::from_serde(&args)?;
        let result = self.client.invoke_method(method, args).await;
        Ok(result.into_serde::<D>()?)
    }
}
