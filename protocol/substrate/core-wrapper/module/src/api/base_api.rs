use crate::{
    debug,
    error::Error,
    types::metadata::Metadata,
    utils::FromHexStr,
    wrap::{
        imported::http_module,
        HttpHeader,
        HttpModule,
        HttpRequest,
        HttpResponse,
        HttpResponseType,
    },
};
use frame_metadata::RuntimeMetadataPrefixed;
use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use sp_core::{
    Decode,
    H256,
};
use sp_runtime::{
    generic::SignedBlock,
    traits::{
        Block,
        Header,
    },
};
use sp_version::RuntimeVersion;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonReq {
    id: usize,
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonResult {
    id: usize,
    jsonrpc: String,
    result: serde_json::Value,
}

/// This api doesn't need Metadata, Runtime version to work
/// It just fetch the content right away
pub struct BaseApi {
    /// the url of the substrate node we are running the rpc call from
    url: String,
}

impl BaseApi {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// Get the runtime metadata of a substrate node.
    /// This is equivalent to running the following command
    ///
    /// `curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' http://localhost:9933/`
    ///
    /// Which makes an rpc call of a substrate node running locally.
    pub fn fetch_runtime_metadata(
        &self,
    ) -> Result<Option<RuntimeMetadataPrefixed>, Error> {
        let value = self.json_request_value("state_getMetadata", ())?;
        match value {
            Some(value) => {
                let value_str = value
                    .as_str()
                    .expect("Expecting a string value on the result");
                let data = Vec::from_hex(value_str)?;
                let rt_metadata =
                    RuntimeMetadataPrefixed::decode(&mut data.as_slice())?;
                Ok(Some(rt_metadata))
            }
            None => Ok(None),
        }
    }

    /// Get the metadata of the substrate chain
    pub fn fetch_metadata(&self) -> Result<Option<Metadata>, Error> {
        let rt_metadata = self.fetch_runtime_metadata()?;
        match rt_metadata {
            Some(rt_metadata) => {
                let metadata = Metadata::try_from(rt_metadata)?;
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    // curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
    pub fn fetch_rpc_methods(&self) -> Result<Option<Vec<String>>, Error> {
        let value = self.json_request_value("rpc_methods", ())?;
        match value {
            Some(value) => {
                let methods: Vec<String> =
                    serde_json::from_value(value["methods"].clone())?;
                Ok(Some(methods))
            }
            None => Ok(None),
        }
    }

    /// return the block hash of block number `n`
    pub fn fetch_block_hash(&self, n: u32) -> Result<Option<H256>, Error> {
        let value = self.json_request_value("chain_getBlockHash", vec![n])?;

        match value {
            Some(value) => {
                let hash = value.as_str().map(H256::from_hex).transpose()?;
                Ok(hash)
            }
            None => Ok(None),
        }
    }

    /// Fetch a substrate block by number `n`
    pub fn fetch_block<B>(&self, n: u32) -> Result<Option<B>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let signed_block = self.fetch_signed_block(n)?;
        Ok(signed_block.map(|sb| sb.block))
    }

    pub fn fetch_genesis_hash(&self) -> Result<Option<H256>, Error> {
        self.fetch_block_hash(0)
    }

    /// Fetch a substrate signed block by number `n`
    pub fn fetch_signed_block<B>(
        &self,
        n: u32,
    ) -> Result<Option<SignedBlock<B>>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let hash = self.fetch_block_hash(n)?;
        if let Some(hash) = hash {
            let block = self.fetch_signed_block_by_hash(hash)?;
            Ok(block)
        } else {
            Ok(None)
        }
    }

    pub fn fetch_finalized_head(&self) -> Result<Option<H256>, Error> {
        let value = self.json_request_value("chain_getFinalizedHead", ())?;
        match value {
            Some(value) => {
                let value_str = value.as_str().expect("Expecting a string");
                Ok(Some(H256::from_hex(value_str)?))
            }
            None => Ok(None),
        }
    }

    pub fn fetch_header<H>(&self, hash: H256) -> Result<Option<H>, Error>
    where
        H: Header + DeserializeOwned,
    {
        let value = self.json_request_value("chain_getHeader", vec![hash])?;
        match value {
            Some(value) => {
                println!("value: {:?}", value);
                Ok(Some(serde_json::from_value(value)?))
            }
            None => Ok(None),
        }
    }

    /// Fetch a substrate block by its hash `hash`
    pub fn fetch_signed_block_by_hash<B>(
        &self,
        hash: H256,
    ) -> Result<Option<SignedBlock<B>>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let value = self.fetch_opaque_block_by_hash(hash)?;
        match value {
            Some(value) => Ok(serde_json::from_value(value)?),
            None => Ok(None),
        }
    }

    pub fn fetch_opaque_block(
        &self,
        n: u32,
    ) -> Result<Option<serde_json::Value>, Error> {
        let block_hash = self.fetch_block_hash(n)?;
        match block_hash {
            Some(block_hash) => self.fetch_opaque_block_by_hash(block_hash),
            None => Ok(None),
        }
    }

    pub fn fetch_opaque_block_by_hash(
        &self,
        hash: H256,
    ) -> Result<Option<serde_json::Value>, Error> {
        let value = self.json_request_value("chain_getBlock", vec![hash])?;
        Ok(value)
    }

    pub fn fetch_runtime_version(
        &self,
    ) -> Result<Option<RuntimeVersion>, Error> {
        let version = self.json_request_value("state_getRuntimeVersion", ())?;
        match version {
            Some(version) => {
                let rt_version: RuntimeVersion =
                    serde_json::from_value(version)?;
                Ok(Some(rt_version))
            }
            None => Ok(None),
        }
    }

    pub fn author_submit_extrinsic(
        &self,
        hex_extrinsic: String,
    ) -> Result<Option<H256>, Error> {
        let value = self.json_request_value(
            "author_submitExtrinsic",
            vec![hex_extrinsic],
        )?;
        match value {
            Some(value) => {
                let value_str = value.as_str().expect("must be a string");
                Ok(Some(H256::from_hex(value_str)?))
            }
            None => Ok(None),
        }
    }

    /// Make a rpc request and return the result.result if it has value
    pub(crate) fn json_request_value<P: Serialize>(
        &self,
        method: &str,
        params: P,
    ) -> Result<Option<serde_json::Value>, Error> {
        let result = self.json_request(method, params)?;
        if result.result.is_null() {
            Ok(None)
        } else {
            Ok(Some(result.result))
        }
    }

    /// Do the actual rpc call into the substrate node using `reqwest` crate.
    /// Note: reqwest crate can run in a tokio runtime or in webassembly runtime, which is why
    /// we are able to compile this whole library into wasm.
    ///
    fn json_request<P: Serialize>(
        &self,
        method: &str,
        params: P,
    ) -> Result<JsonResult, Error> {
        let param = JsonReq {
            id: 1,
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: serde_json::to_value(params)?,
        };
        let response: Result<Option<HttpResponse>, String> =
            HttpModule::post(&http_module::ArgsPost {
                url: self.url.clone(),
                request: Some(HttpRequest {
                    response_type: HttpResponseType::TEXT,
                    headers: Some(vec![HttpHeader {
                        key: String::from("Content-Type"),
                        value: String::from("application/json"),
                    }]),
                    url_params: None,
                    body: Some(serde_json::to_string(&param)?),
                }),
            });

        let response = match response {
            Ok(response) => response,
            Err(err) => return Err(Error::HttpRequestError(err)),
        };

        match response {
            Some(response) => {
                match response.body {
                    Some(body) => Ok(serde_json::from_str(&body)?),
                    None => Err(Error::NoResponse),
                }
            }
            None => Err(Error::NoResponse),
        }
    }
}
