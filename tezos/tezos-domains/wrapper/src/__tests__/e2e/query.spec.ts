import path from "path"
import { Web3ApiClient } from "@web3api/client-js"
import { buildAndDeployApi, initTestEnvironment, stopTestEnvironment } from "@web3api/test-env-js"

import * as QuerySchema from "../../query/w3"
import { getPlugins } from "../testUtils"

jest.setTimeout(150000)

describe("Query", () => {
  let client: Web3ApiClient;
  let ensUri: string;

  beforeAll(async () => {
    const { ensAddress, ethereum, ipfs } = await initTestEnvironment();
    const apiPath = path.join(__dirname, "/../../../");
    const api = await buildAndDeployApi(apiPath, ipfs, ensAddress);
    ensUri = `ens/testnet/${api.ensDomain}`;
    client = new Web3ApiClient({
        plugins: getPlugins(ipfs, ensAddress, ethereum),
    })
  })

  afterAll(async () => {
    await stopTestEnvironment()
  })

  describe("resolveDomain", () => {
    it("should resolve a valid domain name", async () => {
      const response =  await client.query<{ resolveDomain: QuerySchema.DomainInfo | null }>({
        uri: ensUri,
        query: `
          query {
            resolveDomain(
              network: mainnet,
              domain: $domain
            )
          }
        `,
        variables: {
          domain: "alice.tez"
        }
      })
  
      expect(response.errors).toBeUndefined()
      expect(response.data).toBeDefined()
      expect(response.data?.resolveDomain).toBeDefined()
      expect(response.data?.resolveDomain?.Name).toBeDefined()
      expect(response.data?.resolveDomain?.Address).toBeDefined()
      expect(response.data?.resolveDomain?.Data).toBeDefined()
      expect(response.data?.resolveDomain?.Expiry).toBeDefined()
    })
  
    it("should return null for an invalid domain name", async () => {
      const response =  await client.query<{ resolveDomain: QuerySchema.DomainInfo | null }>({
        uri: ensUri,
        query: `
          query {
            resolveDomain(
              network: mainnet,
              domain: $domain
            )
          }
        `,
        variables: {
          domain: `chalak-${Math.random() * 1000}.tez`
        }
      })
  
      expect(response.errors).toBeUndefined()
      expect(response.data).toBeDefined()
      expect(response.data?.resolveDomain).toBeNull()
    })
  })

  describe("resolveAddress", () => {
    it("should resolve address to domain record", async () => {
      const response =  await client.query<{ resolveAddress: QuerySchema.DomainInfo | null }>({
        uri: ensUri,
        query: `
          query {
            resolveAddress(
              network: mainnet,
              address: $address
            )
          }
        `,
        variables: {
          address: 'tz1PnpYYdcgoVq1RYgj6qSdbzwSJRXXcfU3F'
        }
      })

      expect(response.errors).toBeUndefined()
      expect(response.data).toBeDefined()
      expect(response.data?.resolveAddress).toBeDefined()
      expect(response.data?.resolveAddress?.Name).toBeDefined()
      expect(response.data?.resolveAddress?.Address).toBeDefined()
      expect(response.data?.resolveAddress?.Data).toBeDefined()
      expect(response.data?.resolveAddress?.Expiry).toBeDefined()
    })
  })

  describe("generateNonce", () => {
    it.only("should generate a random number", async () => {
      const uniqueNums = new Set()
      for (let i = 0; i < 5; i++) {
        const response =  await client.query<{ generateNonce: number }>({
          uri: ensUri,
          query: `
            query {
              generateNonce
            }
          `
        })

        expect(response.errors).toBeUndefined()
        expect(response.data).toBeDefined()
        expect(uniqueNums).not.toContain(response.data?.generateNonce)
        
        uniqueNums.add(response.data?.generateNonce)
      }
    })
  })
})