/* eslint-disable @typescript-eslint/naming-convention */
import {
  ChainId,
  Ethereum_Module,
  Ethereum_TxResponse,
  GasOptions,
  getChainIdKey,
  Args_approve,
  Args_execCall,
  MethodParameters,
  Ethereum_TxReceipt,
  Ethereum_TxRequest,
} from "../wrap";
import { MAX_UINT_256, NFPM_ADDRESS, ROUTER_ADDRESS } from "../utils";
import { toHex } from "../router";

import { BigInt } from "@polywrap/wasm-as";

export function execCall(args: Args_execCall): Ethereum_TxResponse {
  const methodParameters: MethodParameters = args.parameters;
  const chainId: ChainId = args.chainId;
  const address: string = args.address;
  const gasOptions: GasOptions | null = args.gasOptions;

  return Ethereum_Module.sendTransaction({
    tx: {
      to: address,
      _from: null,
      nonce: null,
      gasLimit: gasOptions === null ? null : gasOptions.gasLimit,
      gasPrice: gasOptions === null ? null : gasOptions.gasPrice,
      data: methodParameters.calldata,
      value: BigInt.fromString(methodParameters.value, 16),
      chainId: null,
      _type: null,
    },
    connection: {
      node: null,
      networkNameOrChainId: getChainIdKey(chainId),
    },
  }).unwrap();
}

export function approve(args: Args_approve): Ethereum_TxResponse {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const amount: BigInt = args.amount === null ? MAX_UINT_256 : args.amount!;
  const gasOptions: GasOptions | null = args.gasOptions;

  return Ethereum_Module.callContractMethod({
    address: args.token.address,
    method:
      "function approve(address spender, uint value) external returns (bool)",
    args: [ROUTER_ADDRESS, toHex({ value: amount })],
    connection: {
      node: null,
      networkNameOrChainId: getChainIdKey(args.token.chainId),
    },
    txOverrides: {
      value: null,
      gasLimit: gasOptions === null ? null : gasOptions.gasLimit,
      gasPrice: gasOptions === null ? null : gasOptions.gasPrice,
    },
  }).unwrap();
}

export function approveNFPM(args: Args_approve): Ethereum_TxReceipt {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const amount = MAX_UINT_256
  const gasOptions: GasOptions | null = args.gasOptions;

  const calldata = Ethereum_Module.encodeFunction({ 
    method: "function approve(address to, uint256 tokenId) external",
    args: [NFPM_ADDRESS, toHex({ value: amount })]
  }).unwrap();

  const signer = Ethereum_Module.getSignerAddress({
    connection: {
      node: null,
      networkNameOrChainId: getChainIdKey(args.token.chainId),
    }
  }).unwrap();

  const approveTransaction: Ethereum_TxRequest = {
    to: args.token.address,
    data: calldata,
    gasLimit: gasOptions === null ? null : gasOptions.gasLimit,
    gasPrice: gasOptions === null ? null : gasOptions.gasPrice,
    chainId: null,
    _type: null,
    value: null,
    _from: signer,
    nonce: null,
  }
  return Ethereum_Module.sendTransactionAndWait({
    tx: approveTransaction,
    connection: {
      node: null,
      networkNameOrChainId: getChainIdKey(args.token.chainId),
    }
  }).unwrap();
}