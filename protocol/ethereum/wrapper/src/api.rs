use ethers_core::{
    abi::{encode, Abi, AbiParser, HumanReadableParser, ParamType, Token},
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, BlockNumber, Bytes, Signature,
        Transaction, TransactionReceipt, TransactionRequest, H256, U256, Eip1559TransactionRequest, NameOrAddress,
    },
    utils::{format_ether, parse_ether, serialize},
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, Provider};
use ethers_signers::Signer;

use crate::error::WrapperError;
use crate::format::{params_to_types, tokenize_values};
use crate::provider::PolywrapProvider;
use crate::signer::PolywrapSigner;
use crate::mapping::EthersTxOptions;

pub async fn get_chain_id() -> U256 {
    let provider = Provider::new(PolywrapProvider {});
    let id: U256 = provider.get_chainid().await.unwrap();
    id
}
pub async fn get_balance(address: Address) -> U256 {
    let provider = Provider::new(PolywrapProvider {});

    let block_tag: BlockId = BlockNumber::Latest.into();

    let balance = provider
        .get_balance(address, Some(block_tag))
        .await
        .unwrap();
    balance
}

pub async fn get_gas_price() -> U256 {
    let provider = Provider::new(PolywrapProvider {});
    let price = provider.get_gas_price().await.unwrap();
    price
}

pub fn get_signer_address() -> Address {
    PolywrapSigner::new().address
}

pub async fn get_signer_balance() -> U256 {
    let wallet = PolywrapSigner::new();
    let address = wallet.address;
    let provider = Provider::new(PolywrapProvider {});
    let block_tag: BlockId = BlockNumber::Latest.into();
    let balance = provider
        .get_balance(address, Some(block_tag))
        .await
        .unwrap();
    balance
}

pub async fn get_signer_transaction_count() -> U256 {
    let wallet = PolywrapSigner::new();
    let address = wallet.address;
    let provider = Provider::new(PolywrapProvider {});
    let block_tag: BlockId = BlockNumber::Latest.into();
    let count = provider
        .get_transaction_count(address, Some(block_tag))
        .await
        .unwrap();
    count
}

pub async fn sign_message(message: &str) -> Signature {
    let wallet = PolywrapSigner::new();
    let signature: Signature = wallet.sign_message(message).await.unwrap();
    signature
}

pub fn encode_params(types: Vec<String>, values: Vec<String>) -> Vec<u8> {
    let kinds: Vec<ParamType> = types
        .iter()
        .map(|t| HumanReadableParser::parse_type(&t).unwrap())
        .collect();
    let tokens: Vec<Token> = tokenize_values(&values, &kinds);
    let bytes = encode(&tokens);
    bytes
}

pub fn encode_function(method: &str, args: Vec<String>) -> Vec<u8> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let bytes = function.encode_input(&tokens).unwrap();
    bytes
}

pub fn decode_function(method: &str, data: Vec<u8>) -> Vec<Token> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let sig = function.short_signature();
    let mut has_sig = false;

    if data[0..4] == sig {
        has_sig = true;
    }

    let arg_bytes: &[u8] = match has_sig {
        true => &data[4..],
        false => &data[0..]
    };

    function.decode_input(arg_bytes).unwrap()
}

pub fn to_wei(eth: String) -> U256 {
    match parse_ether(eth) {
        Ok(wei) => wei,
        Err(e) => panic!("{}", e.to_string()),
    }
}

pub fn to_eth(wei: String) -> U256 {
    let wei = match U256::from_dec_str(&wei) {
        Ok(w) => w,
        Err(_) => panic!("Invalid Wei number: {}", wei),
    };
    format_ether(wei)
}

pub async fn send_rpc(method: &str, params: Vec<String>) -> String {
    let provider = Provider::new(PolywrapProvider {});
    let res: serde_json::Value = provider.request(method, params).await.unwrap();
    res.to_string()
}

pub async fn estimate_transaction_gas(tx: TypedTransaction) -> U256 {
    let provider = Provider::new(PolywrapProvider {});
    let gas = provider.estimate_gas(&tx).await.unwrap();
    gas
}

pub async fn await_transaction(tx_hash: H256) -> Transaction {
    let provider = Provider::new(PolywrapProvider {});
    let response = provider.get_transaction(tx_hash).await.unwrap().unwrap();
    response
}

pub async fn get_transaction_receipt(tx_hash: H256) -> TransactionReceipt {
    let provider = Provider::new(PolywrapProvider {});
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    receipt
}

pub async fn sign_and_send_transaction(client: SignerMiddleware<Provider<PolywrapProvider>, PolywrapSigner>, mut tx: TypedTransaction) -> H256 {
    let address = client.signer().address;
    client.fill_transaction(&mut tx, None).await.unwrap();
    let signature = client.sign_transaction(&tx, address).await.unwrap();
    let signed_tx: Bytes = tx.rlp_signed(&signature);
    let rlp = serialize(&signed_tx);
    let tx_hash: H256 = client
        .inner()
        .request("eth_sendRawTransaction", [rlp])
        .await
        .unwrap();
    tx_hash
}

pub fn create_deploy_contract_transaction(
    abi: Abi,
    bytecode: Bytes,
    params: Vec<String>,
    options: &EthersTxOptions
) -> Result<TypedTransaction, WrapperError> {
    let data: Bytes = match (abi.constructor(), params.is_empty()) {
        (None, false) => {
            return Err(WrapperError::ContractError(
                ethers_contract::ContractError::ConstructorError,
            ))
        }
        (None, true) => bytecode.clone(),
        (Some(constructor), _) => {
            let kinds: Vec<ParamType> = params_to_types(&constructor.inputs);
            let tokens: Vec<Token> = tokenize_values(&params, &kinds);
            constructor.encode_input(bytecode.to_vec(), &tokens)?.into()
        }
    };
    let tx: TypedTransaction = create_transaction(None, data, options);
    Ok(tx)
}

pub async fn estimate_contract_call_gas(address: Address, method: &str, args: &Vec<String>, options: &EthersTxOptions) -> U256 {
    let provider = Provider::new(PolywrapProvider {});
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();

    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);

    let gas = provider.estimate_gas(&tx).await.unwrap();
    gas
}

pub async fn call_contract_view(address: Address, method: &str, args: Vec<String>) -> Vec<Token> {
    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);
    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();

    let tx: TypedTransaction = TransactionRequest {
        to: Some(address.into()),
        data: Some(data),
        ..Default::default()
    }.into();

    let provider = Provider::new(PolywrapProvider {});
    let bytes = provider.call(&tx, None).await.unwrap();
    let tokens: Vec<Token> = function.decode_output(&bytes).unwrap();
    tokens
}

pub async fn call_contract_static(
    address: Address,
    method: &str,
    args: Vec<String>,
    options: &EthersTxOptions,
) -> Result<Vec<Token>, WrapperError> {
    let provider = Provider::new(PolywrapProvider {});
    let wallet = PolywrapSigner::new();
    let client = SignerMiddleware::new(provider, wallet);

    let function = AbiParser::default().parse_function(method)?;
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);

    let data: Bytes = function.encode_input(&tokens).map(Into::into)?;

    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);

    client.fill_transaction(&mut tx, None).await?;
    let bytes = client.inner().call(&tx, None).await?;
    let tokens: Vec<Token> = function.decode_output(&bytes)?;
    Ok(tokens)
}

pub async fn call_contract_method(
    address: Address,
    method: &str,
    args: Vec<String>,
    options: &EthersTxOptions,
) -> H256 {
    let provider = Provider::new(PolywrapProvider {});
    let wallet = PolywrapSigner::new();
    let client = SignerMiddleware::new(provider, wallet);

    let function = AbiParser::default().parse_function(method).unwrap();
    let kinds: Vec<ParamType> = params_to_types(&function.inputs);
    let tokens: Vec<Token> = tokenize_values(&args, &kinds);

    let data: Bytes = function.encode_input(&tokens).map(Into::into).unwrap();

    let mut tx: TypedTransaction = create_transaction(Some(address), data, options);

    let tx_hash: H256 = sign_and_send_transaction(client, tx).await;
    tx_hash
}

fn create_transaction(address: Option<Address>, data: Bytes, options: &EthersTxOptions) -> TypedTransaction {
    if options.no_eip1559 {
        return TransactionRequest {
            to: address.map(Into::into),
            data: Some(data),
            gas: options.gas_limit,
            gas_price: options.max_fee_per_gas,
            value: options.value,
            nonce: options.nonce,
            ..Default::default()
        }.into();
    }
    Eip1559TransactionRequest {
        to: address.map(Into::into),
        data: Some(data),
        gas: options.gas_limit,
        max_fee_per_gas: options.max_fee_per_gas,
        max_priority_fee_per_gas: options.max_priority_fee_per_gas,
        value: options.value,
        nonce: options.nonce,
        ..Default::default()
    }.into()
}