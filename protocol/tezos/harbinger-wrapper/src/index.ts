import {
  Tezos_Module,
  Args_getAssetData,
  Args_getNormalizedPrice,
  Providers,
  Args_listAssets,
  Args_listProviders,
  Args_getCandle,
  AssetCandle,
} from "./wrap";
import { 
  getString, 
  getConnection,
  normalizeValue,
} from "./utils/common"

import { JSON } from "assemblyscript-json"; 

// Provide default oracle contract addresses for network
// Link for contract references: https://github.com/tacoinfra/harbinger
// Mainnet contract: KT1Jr5t9UvGiqkvvsuUbPJHaYx24NzdUwNW9

export function listProviders(_: Args_listProviders): Providers[] {
  return [
    {
      Provider: "Coinbase",
      ProviderNetworks: [
        {
          Network: "mainnet",
          Kind: "Storage",
          ContractAddress: "KT1Jr5t9UvGiqkvvsuUbPJHaYx24NzdUwNW9"
        },
        {
          Network: "mainnet",
          Kind: "Normalizer",
          ContractAddress: "KT1AdbYiPYb5hDuEuVrfxmFehtnBCXv4Np7r"
        }
      ]
    },
    {
      Provider: "Binance",
      ProviderNetworks: [
        {
          Network: "mainnet",
          Kind: "Storage",
          ContractAddress: "KT1Mx5sFU4BZqnAaJRpMzqaPbd2qMCFmcqea"
        },
        {
          Network: "mainnet",
          Kind: "Normalizer",
          ContractAddress: "KT1SpD9Xh3PcmBGwbZPhVmHUM8shTwYhQFBa"
        }
      ]
    },
    {
      Provider: "OKEx",
      ProviderNetworks: [
        {
          Network: "mainnet",
          Kind: "Storage",
          ContractAddress: "KT1G3UMEkhxso5cdx2fvoJRJu5nUjBWKMrET"
        },
        {
          Network: "mainnet",
          Kind: "Normalizer",
          ContractAddress: "KT1SUP27JhX24Kvr11oUdWswk7FnCW78ZyUn"
        }
      ]
    },
  ]
};

export function listAssets(input: Args_listAssets): string {
  const connectionDetails = getConnection(input.network, input.providerAddress, input.custom);
  return Tezos_Module.getContractStorage({
    address: connectionDetails.contractAddress,
    connection: connectionDetails.connection,
    key: "assetCodes",
    field: ""
  }).unwrap();
}

export function getCandle(input: Args_getCandle): AssetCandle {
  const connectionDetails = getConnection(input.network, input.providerAddress, input.custom);
  const storage = Tezos_Module.getContractStorage({
    address: connectionDetails.contractAddress,
    connection: connectionDetails.connection,
    key: "oracleData",
    field: input.assetCode
  }).unwrap();
  const assetData = <JSON.Obj>JSON.parse(storage);
  return {
      low:  normalizeValue(parseFloat(getString(assetData, "4"))),
      open: normalizeValue(parseFloat(getString(assetData, "2"))),
      high: normalizeValue(parseFloat(getString(assetData, "3"))),
      asset: input.assetCode,
      close: normalizeValue(parseFloat(getString(assetData, "5"))),
      volume: normalizeValue(parseFloat(getString(assetData, "6"))),
      endPeriod: getString(assetData, "1"),
      startPeriod: getString(assetData, "0"),
  };
}

export function getNormalizedPrice(input: Args_getNormalizedPrice): string {
  const connectionDetails = getConnection(input.network, input.providerAddress, input.custom);
  const storage = Tezos_Module.getContractStorage({
    address: connectionDetails.contractAddress,
    connection: connectionDetails.connection,
    key: "assetMap",
    field: input.assetCode
  }).unwrap();
  const assetData = <JSON.Obj>JSON.parse(storage);
  return getString(assetData, "computedPrice")
}

export function getAssetData(input: Args_getAssetData): AssetCandle {
  const connectionDetails = getConnection(input.network, input.providerAddress, input.custom);
  const storageValue = Tezos_Module.getContractStorage({
    address: connectionDetails.contractAddress,
    connection: connectionDetails.connection,
    key: "oracleData",
    field: input.assetCode
  }).unwrap();
  const assetData = <JSON.Obj>JSON.parse(storageValue);
  return {
      low:  normalizeValue(parseFloat(getString(assetData, "4"))),
      open: normalizeValue(parseFloat(getString(assetData, "2"))),
      high: normalizeValue(parseFloat(getString(assetData, "3"))),
      asset: input.assetCode,
      close: normalizeValue(parseFloat(getString(assetData, "5"))),
      volume: normalizeValue(parseFloat(getString(assetData, "6"))),
      endPeriod: getString(assetData, "1"),
      startPeriod: getString(assetData, "0"),
  };
}