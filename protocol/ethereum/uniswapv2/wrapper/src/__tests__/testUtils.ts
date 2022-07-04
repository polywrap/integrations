import { BestTradeOptions, ChainId, Pair, Token, TokenAmount, Trade, TxResponse, TradeOptions } from "./e2e/types";
import { ClientConfig, coreInterfaceUris, Web3ApiClient } from "@web3api/client-js";
import { ethereumPlugin } from "@web3api/ethereum-plugin-js";
import { ipfsPlugin } from "@web3api/ipfs-plugin-js";
import { ensPlugin } from "@web3api/ens-plugin-js";
import * as uni from "@uniswap/sdk";
import tokenList from "./e2e/testData/tokenList.json";

export function getPlugins(ethereum: string, ipfs: string, ensAddress: string): ClientConfig {
  return {
    redirects: [],
    plugins: [
     {
       uri: "w3://ens/ipfs.web3api.eth",
       plugin: ipfsPlugin({ provider: ipfs }),
     },
     {
       uri: "w3://ens/ens.web3api.eth",
       plugin: ensPlugin({ query: { addresses: { testnet: ensAddress } } }),
     },
     {
      uri: "w3://ens/ethereum.web3api.eth",
      plugin: ethereumPlugin({
          networks: {
            testnet: {
              provider: ethereum
            },
            MAINNET: {
              provider: "http://localhost:8546"
            },
          },
          defaultNetwork: "testnet"
        }),
      },
    ],
    interfaces: [
      {
        interface: coreInterfaceUris.uriResolver.uri,
        implementations: [
          "w3://ens/ipfs.web3api.eth",
          "w3://ens/ens.web3api.eth",
        ],
      },
      {
        interface: coreInterfaceUris.logger.uri,
        implementations: ["w3://ens/js-logger.web3api.eth"],
      },
    ],
    envs: [],
    uriResolvers: []
  };
}

export async function getTokenList(): Promise<Token[]> {
  let tokens: Token[] = [];
  tokenList.forEach((token: {
    address: string;
    decimals: number;
    symbol: string;
    name: string;
  }) => tokens.push({
    chainId: ChainId.MAINNET,
    address: token.address,
    currency: {
      decimals: token.decimals,
      symbol: token.symbol,
      name: token.name,
    },
  }));
  return tokens;
}

export async function getPairData(token0: Token, token1: Token, client: Web3ApiClient, ensUri: string): Promise<Pair | undefined> {
  const pairData = await client.query<{
    fetchPairData: Pair;
  }>({
    uri: ensUri,
    query: `
        query {
          fetchPairData(
            token0: $token0
            token1: $token1
          )
        }
      `,
    variables: {
      token0: token0,
      token1: token1
    },
  });

  if (pairData.errors) {
    throw pairData.errors;
  }

  return pairData.data?.fetchPairData;
}

export function getUniPairs(pairs: Pair[], chainId: number): uni.Pair[] {
  return pairs.map(pair => {
    return new uni.Pair(
      new uni.TokenAmount(
        new uni.Token(
          chainId,
          pair.tokenAmount0.token.address,
          pair.tokenAmount0.token.currency.decimals,
          pair.tokenAmount0.token.currency.symbol || "",
          pair.tokenAmount0.token.currency.name || ""
        ),
        pair.tokenAmount0.amount
      ),
      new uni.TokenAmount(
        new uni.Token(
          chainId,
          pair.tokenAmount1.token.address,
          pair.tokenAmount1.token.currency.decimals,
          pair.tokenAmount1.token.currency.symbol || "",
          pair.tokenAmount1.token.currency.name || ""
        ),
        pair.tokenAmount1.amount
      ),
    );
  });
}

export async function getBestTradeExactIn(
  allowedPairs: Pair[],
  currencyAmountIn: TokenAmount,
  currencyOut: Token,
  bestTradeOptions: BestTradeOptions | null,
  client: Web3ApiClient,
  ensUri: string
): Promise<Trade[]> {
  const query = await client.query<{
    bestTradeExactIn: Trade[]
  }>({
    uri: ensUri,
    query: `query {
        bestTradeExactIn(
          pairs: $pairs
          amountIn: $amountIn
          tokenOut: $tokenOut
          options: ${bestTradeOptions ?? "null"}
         )
       }`,
    variables: {
      pairs: allowedPairs,
      amountIn: currencyAmountIn,
      tokenOut: currencyOut,
    }
  })
  const result: Trade[] | undefined = query.data?.bestTradeExactIn
  if (query.errors) {
    query.errors.forEach(console.log)
  }
  return result!
}

export async function getBestTradeExactOut(
  allowedPairs: Pair[],
  currencyIn: Token,
  currencyAmountOut: TokenAmount,
  bestTradeOptions: BestTradeOptions | null,
  client: Web3ApiClient,
  ensUri: string
): Promise<Trade[]> {
  const query = await client.query<{
    bestTradeExactOut: Trade[]
  }>({
    uri: ensUri,
    query: `query {
        bestTradeExactOut(
          pairs: $pairs
          tokenIn: $tokenIn
          amountOut: $amountOut
          options: ${bestTradeOptions ?? "null"}
         )
       }`,
    variables: {
      pairs: allowedPairs,
      tokenIn: currencyIn,
      amountOut: currencyAmountOut,
    }
  })
  const result: Trade[] | undefined = query.data?.bestTradeExactOut
  if (query.errors) {
    query.errors.forEach(console.log)
  }
  return result!
}

export async function approveToken(
  token: Token,
  client: Web3ApiClient,
  ensUri: string
): Promise<TxResponse> {
  const query = await client.query<{approve: TxResponse}>({
    uri: ensUri,
    query: `
        mutation {
          approve(
            token: $token
          )
        }
      `,
    variables: {
      token: token,
    },
  });
  const result: TxResponse | undefined = query.data?.approve
  if (query.errors) {
    query.errors.forEach(console.log)
  }
  return result!;
}

export async function execTrade(
  trade: Trade,
  tradeOptions: TradeOptions,
  client: Web3ApiClient,
  ensUri: string
): Promise<TxResponse> {
  const query = await client.query<{ exec: TxResponse}>({
    uri: ensUri,
    query: `
          mutation {
            exec (
              trade: $trade
              tradeOptions: $tradeOptions
            )
          }
        `,
    variables: {
      trade: trade,
      tradeOptions: tradeOptions,
    },
  });
  const result: TxResponse | undefined = query.data?.exec
  if (query.errors) {
    query.errors.forEach(console.log)
  }
  return result!;
}

export async function execSwap(
  tokenIn: Token,
  tokenOut: Token,
  amount: string,
  tradeType: string,
  tradeOptions: TradeOptions,
  client: Web3ApiClient,
  ensUri: string
): Promise<TxResponse> {
  const query = await client.query<{ swap: TxResponse}>({
    uri: ensUri,
    query: `
        mutation {
          swap (
            tokenIn: $token0
            tokenOut: $token1
            amount: $amount
            tradeType: $tradeType
            tradeOptions: $tradeOptions
          )
        }
      `,
    variables: {
      token0: tokenIn,
      token1: tokenOut,
      amount: amount,
      tradeType: tradeType,
      tradeOptions: tradeOptions
    },
  });
  const result: TxResponse | undefined = query.data?.swap
  if (query.errors) {
    query.errors.forEach(console.log)
  }
  return result!;
}
