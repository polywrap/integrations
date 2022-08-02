import { Substrate_Module } from "./wrap";

import { Uri, PolywrapClient } from "@polywrap/client-js";
import { runCLI } from "@polywrap/test-env-js";
import path from "path";

jest.setTimeout(360000);

describe("e2e", () => {
  let client: PolywrapClient;
  let uri: string;

  beforeAll(async () => {

    const wrapperDir = path.resolve(__dirname, "../");

    const buildOutput = await runCLI({
      args: ["build"],
      cwd: wrapperDir
    });

    if (buildOutput.exitCode !== 0) {
      throw Error(
        `Failed to build wrapper:\n` +
        JSON.stringify(buildOutput, null, 2)
      );
    }

    uri = new Uri(`fs/${wrapperDir}/build`).uri;
    client = new PolywrapClient();
  });

  it("blockHash", async () => {
      console.log("uri: ", uri);
    // You can use the client directly
    let result_alt = await client.invoke({
      uri,
      method: "blockHash",
        args: {
          url: "http://localhost:9933",
          number: 0
        }
    });

    // Or use the test app's codegen types (see web3api.app.yaml)
    const result = await Substrate_Module.blockHash(
      {
          url: "http://localhost:9933",
          number: 0
      },
      client,
      uri
    );


      console.log("result:", result);
      console.log("result_alt: ", result_alt);

      expect(JSON.stringify(result_alt) == JSON.stringify(result)).toBe(true);
  });

  it("block", async () => {
    // You can use the client directly
    const result_alt = await client.invoke({
      uri,
      method: "chainGetBlock",
        args: {
          url: "http://localhost:9933",
          number: 0
        },
    });

    // Or use the test app's codegen types (see web3api.app.yaml)
    const result = await Substrate_Module.chainGetBlock(
      {
          url: "http://localhost:9933",
          number: 0
      },
      client,
      uri
    );

      console.log("block result:", result);
      expect(JSON.stringify(result_alt) == JSON.stringify(result)).toBe(true);

  });

  it("chainGetMetadata", async () => {
    // You can use the client directly
    const result_alt = await client.invoke({
      uri,
      method: "chainGetMetadata",
        args: {
          url: "http://localhost:9933"
        }
    });

    // Or use the test app's codegen types (see web3api.app.yaml)
    const result = await Substrate_Module.chainGetMetadata(
      {
          url: "http://localhost:9933"
      },
      client,
      uri
    );

      console.log("http response: ", result);
      expect(JSON.stringify(result_alt) == JSON.stringify(result)).toBe(true);
  });

  it("state get runtime version", async () => {
    // You can use the client directly
    const result = await client.invoke({
      uri,
      method: "stateGetRuntimeVersion",
        args: {
          url: "http://localhost:9933"
        }
    });

      console.log("runtime version: ", result);
  });


  it("storage value", async () => {
    // You can use the client directly
    const result_alt = await client.invoke({
      uri,
      method: "getStorageValue",
      args: {
          url: "http://localhost:9933",
          pallet: "TemplateModule",
          storage: "Something"
      },
    });

    // Or use the test app's codegen types (see web3api.app.yaml)
    const result = await Substrate_Module.getStorageValue(
      {
          url: "http://localhost:9933",
          pallet: "TemplateModule",
          storage: "Something"
      },
      client,
      uri
    );

      console.log("template module Something:", result);
      expect(JSON.stringify(result_alt) == JSON.stringify(result)).toBe(true);

  });

  it("return constant values", async () => {
    // You can use the client directly
    const existentialDeposit = await client.invoke({
      uri,
      method: "constant",
      args: {
          url: "http://localhost:9933",
          pallet: "Balances",
          name: "ExistentialDeposit"
      },
    });
      console.log("Balances ExistentialDeposit", existentialDeposit);
  });

  it("rpc_methods", async () => {
    // You can use the client directly
    const result = await client.invoke({
      uri,
      method: "rpcMethods",
      args: {
          url: "http://localhost:9933",
      },
    });
      console.log("result", result);
  });

  it("get storage maps", async () => {
    // You can use the client directly
    const result = await client.invoke({
      uri,
      method: "getStorageMap",
      args: {
          url: "http://localhost:9933",
          pallet: "ForumModule",
          storage: "AllPosts",
          key: "0",
      },
    });
      console.log("result", result);
  });


  it("get storage maps paged", async () => {
    // You can use the client directly
    const result = await client.invoke({
      uri,
      method: "getStorageMapPaged",
      args: {
          url: "http://localhost:9933",
          pallet: "ForumModule",
          storage: "AllPosts",
          count: 10,
          nextTo: null,
      },
    });
      console.log("paged storage map", result);
  });

  it("get account info of Alice", async () => {
    // You can use the client directly
    const result = await client.invoke({
      uri,
      method: "accountInfo",
      args: {
          url: "http://localhost:9933",
          account: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      },
    });
      console.log("alice account info", result);
  });

});
