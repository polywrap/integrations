import { ChainId, Currency, Token, TokenAmount } from "../../../wrap";
import { getNative, getWrappedNative, isNative, wrapAmount, wrapToken } from "../../..";
import { BigInt } from "@polywrap/wasm-as";

const Eth: Currency = {
  decimals: 18,
  name: "Ether",
  symbol: "ETH",
};
const WETH: Currency = {
  decimals: 18,
  symbol: "WETH",
  name: "Wrapped Ether",
};
export const MATIC: Currency = {
  decimals: 18,
  name: "Polygon Matic",
  symbol: "MATIC",
};
export const WMATIC: Currency = {
  decimals: 18,
  name: "Wrapped MATIC",
  symbol: "WMATIC",
};

const USDC: Token = {
  chainId: ChainId.MAINNET,
  address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  currency: {
    decimals: 6,
    symbol: "USDC",
    name: "USD Coin",
  },
};

describe('Token utils', () => {

  it('getNative', () => {
    for (let i = ChainId.MAINNET; i < ChainId._MAX_; i++) {
      let currency: Currency;
      if (i == ChainId.POLYGON || i == ChainId.POLYGON_MUMBAI) {
        currency = MATIC;
      } else {
        currency = Eth;
      }
      const result: Token = {
        chainId: i as ChainId,
        address: "",
        currency: currency,
      };

      expect(getNative({ chainId: i as ChainId })).toStrictEqual(result);
    }

    const error = (): void => { getNative({ chainId: 999 }) };
    expect(error).toThrow("Unknown chain ID");
  });

  it('getWrappedNative', () => {
    expect(getWrappedNative({ chainId: ChainId.MAINNET })).toStrictEqual({
      chainId: ChainId.MAINNET,
      address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.ROPSTEN })).toStrictEqual({
      chainId: ChainId.ROPSTEN,
      address: "0xc778417E063141139Fce010982780140Aa0cD5Ab",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.RINKEBY })).toStrictEqual({
      chainId: ChainId.RINKEBY,
      address: "0xc778417E063141139Fce010982780140Aa0cD5Ab",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.GOERLI })).toStrictEqual({
      chainId: ChainId.GOERLI,
      address: "0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.KOVAN })).toStrictEqual({
      chainId: ChainId.KOVAN,
      address: "0xd0A1E359811322d97991E03f863a0C30C2cF029C",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.OPTIMISM })).toStrictEqual({
      chainId: ChainId.OPTIMISM,
      address: "0x4200000000000000000000000000000000000006",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.OPTIMISTIC_KOVAN })).toStrictEqual({
      chainId: ChainId.OPTIMISTIC_KOVAN,
      address: "0x4200000000000000000000000000000000000006",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.ARBITRUM_ONE })).toStrictEqual({
      chainId: ChainId.ARBITRUM_ONE,
      address: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1",
      currency: WETH,
    });
    expect(getWrappedNative({ chainId: ChainId.ARBITRUM_RINKEBY })).toStrictEqual({
      chainId: ChainId.ARBITRUM_RINKEBY,
      address: "0xB47e6A5f8b33b3F17603C83a0535A9dcD7E32681",
      currency: WETH,
    });

    expect(getWrappedNative({ chainId: ChainId.POLYGON })).toStrictEqual({
      chainId: ChainId.POLYGON,
      address: "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
      currency: WMATIC,
    });
    expect(getWrappedNative({ chainId: ChainId.POLYGON_MUMBAI })).toStrictEqual({
      chainId: ChainId.POLYGON_MUMBAI,
      address: "0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889",
      currency: WMATIC,
    });

    const error = (): void => { getWrappedNative({ chainId: 999 }) };
    expect(error).toThrow("Unknown chain ID");
  });

  it('isNative', () => {
    const ethToken: Token = getNative({ chainId: ChainId.MAINNET });
    expect(isNative({ token: ethToken })).toStrictEqual(true);
    expect(isNative({ token: USDC })).toStrictEqual(false);
  });

  it('wrapToken', () => {
    const ethToken: Token = getNative({ chainId: ChainId.MAINNET });
    expect(wrapToken({ token: ethToken })).toStrictEqual(getWrappedNative({ chainId: ethToken.chainId }));
    expect(wrapToken({ token: USDC })).toStrictEqual(USDC);
  });

  it('wrapAmount', () => {
    const ethAmount: TokenAmount = {
      token: getNative({ chainId: ChainId.MAINNET }),
      amount: BigInt.fromUInt16(100),
    };
    const wethAmount: TokenAmount = {
      token: getWrappedNative({ chainId: ethAmount.token.chainId }),
      amount: BigInt.fromUInt16(100),
    };
    expect(wrapAmount({ amount: ethAmount })).toStrictEqual(wethAmount);

    const USDCAmount: TokenAmount = {
      token: USDC,
      amount: BigInt.fromUInt16(100),
    };
    expect(wrapAmount({ amount: USDCAmount })).toStrictEqual(USDCAmount);
  });

});