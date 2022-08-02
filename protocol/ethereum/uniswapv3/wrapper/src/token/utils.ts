import {
  Args_getNative,
  Args_getWrappedNative,
  Args_isNative,
  Args_wrapAmount,
  Args_wrapToken,
  ChainId,
  Currency,
  Token,
  TokenAmount,
} from "../wrap";
import { currencyEquals } from "./index";

export function getNative(args: Args_getNative): Token {
  return _getNative(args.chainId);
}

export function getWrappedNative(args: Args_getWrappedNative): Token {
  return _getWrappedNative(args.chainId);
}

export function isNative(args: Args_isNative): boolean {
  return _isNative(args.token);
}

export function wrapToken(args: Args_wrapToken): Token {
  return _wrapToken(args.token);
}

export function wrapAmount(args: Args_wrapAmount): TokenAmount {
  return _wrapAmount(args.amount);
}

const ETHER: Currency = {
  decimals: 18,
  name: "Ether",
  symbol: "ETH",
};

const wEthCurrency: Currency = {
  decimals: 18,
  name: "Wrapped Ether",
  symbol: "WETH",
};

const MATIC: Currency = {
  decimals: 18,
  name: "Polygon Matic",
  symbol: "MATIC",
};

const wMaticCurrency: Currency = {
  decimals: 18,
  name: "Wrapped MATIC",
  symbol: "WMATIC",
};

function isMatic(chainId: ChainId): boolean {
  return chainId == ChainId.POLYGON || chainId == ChainId.POLYGON_MUMBAI;
}

export function _isNative(token: Token): boolean {
  if (token.address != "") {
    return false;
  }
  const currencyA = token.currency;
  const currencyB = isMatic(token.chainId) ? MATIC : ETHER;
  return currencyEquals({ currencyA, currencyB });
}

function _getNative(chainId: ChainId): Token {
  if (chainId < 0 || chainId >= ChainId._MAX_) {
    throw new Error("Unknown chain ID");
  }
  const currency: Currency = isMatic(chainId)
    ? copyCurrency(MATIC)
    : copyCurrency(ETHER);
  return {
    chainId,
    address: "",
    currency,
  };
}

function _getWrappedNativeAddress(chainId: ChainId): string {
  switch (chainId) {
    case ChainId.MAINNET:
      return "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
    case ChainId.ROPSTEN:
    case ChainId.RINKEBY:
      return "0xc778417E063141139Fce010982780140Aa0cD5Ab";
    case ChainId.GOERLI:
      return "0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6";
    case ChainId.KOVAN:
      return "0xd0A1E359811322d97991E03f863a0C30C2cF029C";
    case ChainId.OPTIMISM:
    case ChainId.OPTIMISTIC_KOVAN:
      return "0x4200000000000000000000000000000000000006";
    case ChainId.ARBITRUM_ONE:
      return "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1";
    case ChainId.ARBITRUM_RINKEBY:
      return "0xB47e6A5f8b33b3F17603C83a0535A9dcD7E32681";
    case ChainId.POLYGON:
      return "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270";
    case ChainId.POLYGON_MUMBAI:
      return "0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889";
    default:
      throw new Error("Unknown chain ID");
  }
}

function _getWrappedNative(chainId: ChainId): Token {
  const address = _getWrappedNativeAddress(chainId);
  const currency = isMatic(chainId)
    ? copyCurrency(wMaticCurrency)
    : copyCurrency(wEthCurrency);
  return { chainId, address, currency };
}

// check if need to wrap ether
export function _wrapToken(token: Token): Token {
  if (_isNative(token)) {
    return _getWrappedNative(token.chainId);
  }
  return token;
}

// check if need to wrap ether
export function _wrapAmount(tokenAmount: TokenAmount): TokenAmount {
  return {
    token: _wrapToken(tokenAmount.token),
    amount: tokenAmount.amount,
  };
}

function copyCurrency(currency: Currency): Currency {
  return {
    name: currency.name,
    symbol: currency.symbol,
    decimals: currency.decimals,
  };
}

function copyToken(token: Token): Token {
  return {
    chainId: token.chainId,
    address: token.address,
    currency: copyCurrency(token.currency),
  };
}

export function copyTokenAmount(tokenAmount: TokenAmount): TokenAmount {
  return {
    token: copyToken(tokenAmount.token),
    amount: tokenAmount.amount.copy(),
  };
}
