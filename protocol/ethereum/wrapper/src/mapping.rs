use super::wrap::imported::{SignerTxRequest};
use ethers_core::{
    types::{
        transaction::{request::TransactionRequest},
        NameOrAddress
    }
};
use polywrap_wasm_rs::BigInt;

pub fn to_tx_request(request: TransactionRequest) -> SignerTxRequest {
    SignerTxRequest {
        to: request.to.map(|f| match f {
          NameOrAddress::Address(addr) => format!("{addr:x}"),
          NameOrAddress::Name(name) => name.to_string(),
        }),
        from: request.from.map(|f| f.to_string()),
        gas_limit: request.gas.map(|f| BigInt::try_from(f.as_u128()).unwrap()),
        gas_price: request
            .gas_price
            .map(|f| BigInt::try_from(f.as_u128()).unwrap()),
        data: request
          .data
          .map(|f| format!("{f:#x}")),
        value: request
            .value
            .map(|f| BigInt::try_from(f.as_u128()).unwrap()),
        nonce: request.nonce.map(|f| f.as_u32()),
        chain_id: Option::None,
        m_type: Option::None
    }
}