# Example app demo for Polywrap with substrate chain

## Forum app
A forum application backed by a blockchain

This forum-app is an example web3 application showcasing the use of polywrap client specific to substrate node.
Additionally, the forum app is written in rust and compiled to wasm and is meant to be run on the browser.
Under the hood, it is using `wasm-bindgen` to manipulate DOM objects from rust and `sauron` web framework to simplify writing the application UI and components lifecycle.


## Prerequisite
Make sure the following cli tools are installed
- docker
- git
- curl
- rust
- wasm-pack

## Frameworks and libraries used
- [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
- [sauron web framework](https://github.com/ivanceras/sauron)

We modified the `substrate-node-template` with the following changes
    - use the token symbol to `CAP`, short for bottlecaps (default is `Unit`).
    - increase the block time production to `1` second (default is `6` seconds) to make adding post and comments faster.
    - We also modify the spec_name and impl_name in the `RuntimeVersion` to "forum-node".
      ```rust
      spec_name: create_runtime_str!("forum-node"),
      impl_name: create_runtime_str!("forum-node"),
      ```

## Demo steps
1. Run the substrate-node-template.
    - `cd substrate-node-template && cargo run --release -- --dev --pruning archive`
    - By default the substrate node just keeps 256 of the latest blocks and the old blocks are discarded.
        - We tell the node to archive the block, so we can reference it later for the demo.

2. Open [polkadot.js.org/apps](https://polkadot.js.org/apps/)
    - This requires to have the [polkadot-js extension](https://polkadot.js.org/extension/) be installed on your browser.
    - Show that we can interact with the `substrate-node-template` with the `ForumModule` storage and extrinsics.

3. Compile the and deploy the `core-wrapper` module of `polywrap-substrate`
    - `cd core-wrapper && yarn && yarn rebuild && yarn deploy`
    - Copy the `ipfs hash` returned in the deploy into the `forum-app/index.ts`

4. Start the forum-app example
    - `cd examples/forum-app && yarn && yarn start`
    - The `forum-app` will be our UI that the user will see.
      The app would be interacting with the modified `substrate-node-template` through polywrap-client to fetch the data stored in the chain storage which will then be displayed in the UI.
    - Take note, that the `forum-app` does not use the `pallet-forum` as it's dependency, neither as the `core-wrapper` module.
        - The `forum-app` will use the `core-wrapper` module through `polywrap` client and dynamically invoke the methods of `pallet-forum`.
        ```bob

         .-------------.                        .---------------.                        .------------------------.                    .----------------.
         |forum-app(UI)|----(wasm-bindgen)----->|polywrap-client|---(polywrap-runtime)-->| substrate core-wrapper |-----(http rpc)---->| substrate node |
         `-------------'                        `---------------'                        `------------------------'                    `----------------'
              (rust)                                  (js)                                      (wasm)                                (rcp-server http://localhost:9933)

        ```
    - A little explanation about the `pallet-forum`
        - `pallet-forum` is implemented with 4 storage in substrate
            1. `AllPosts` - is a storage map keyed by a `post_id(u32)` and `Post` struct as the value.
             ```rust
             #[derive(Encode, Decode, Debug)]
             pub struct Post {
                 pub post_id: u32,
                 pub content: BoundedVec<u8, MaxContentLength>,
                 pub author: AccountId32,
                 pub timestamp: u64,
                 pub block_number: u32,
             }
             ```
            2. `AllComments` - is also a storage map keyed by `comment_id(u32)` and `Comment` struct as the value.
            ```rust
            #[derive(Encode, Decode, Debug)]
            pub struct Comment {
                pub comment_id: u32,
                pub content: BoundedVec<u8, MaxContentLength>,
                pub author: AccountId32,
                pub parent_item: u32,
                pub timestamp: u64,
                pub block_number: u32,
            }
            ```
            3. `Kids` keeps track of the heirarchy of where a comment is replying to which post or another comment.
                - It could be thought as `Map<u32, Vec<u32>>`.
            4. `ItemCounter` we use this to dispense an incrementing counter for both a post or a comment.
                - Ideally, you could a different counter for `Post` and a different one for `Comment`,
                 but this design works well, as you wouldn't same `item_id` for `Post` and `Comment`
     - The `pallet-forum` is added into the `runtime`
        ```rust
        impl pallet_forum::Config for Runtime {
            type Event = Event;
            type MaxContentLength = ConstU32<280>;
            type MaxComments = ConstU32<1000>;
        }

        construct_runtime!(
            pub enum Runtime
            {
                //<...>
                // Include the custom logic from the pallet-template in the runtime.
                TemplateModule: pallet_template,
                ForumModule: pallet_forum,
            }
        );
        ```

    - Post some content and show the console log.
    - Comment on the posts
    - Open the linked block hash where the post or comment is added to the exact block hash.
        - Open the extrinsic details, showing the author and the posted content
    - Run the seed-content to put more content
    - Use the reward button to send tokens to the content authors
        - Show the balances of `Alice` change over time.

5. Explain how the example app is using polywrap-client for substrate.
    - In `index.ts`, we linked this into the page `index.html`

     ```typescript
     let module_uri = "wrap://ipfs/QmQDqV2Y3QD87CG48quXmWXigLBZ3J2qn5z4BY4ZhwMTtL"

     class PolywrapClientWrapper extends PolywrapClient{
         constructor() {
             super({
                 plugins: [
                     {
                       uri: "wrap://ens/ipfs.polywrap.eth",
                       plugin: ipfsPlugin({
                           provider: "https://ipfs.wrappers.io"
                       }),
                     }
                 ]
             })
         }

         async invoke_method(method, args){
             console.log("invoking method:", method);
             console.log("with args:", args);

             let result = await super.invoke({
                 uri: module_uri,
                 method,
                 args,
             });

             console.log("got result: ", result);
             return result.data;
         }
     }
     // attach the PolywrapClient to the window so we can access it later in the rust code
     window.PolywrapClientWrapper = PolywrapClientWrapper;
     ```

    - In the rust side of things, we use `wasm-bindgen` to generate the necessary glue code to connect the `PolywrapClientWrapper` rust object into its javascript object counterpart.

      ```rust
      #[wasm_bindgen]
      extern "C" {

          #[derive(Debug, Clone)]
          pub type PolywrapClientWrapper;

          #[wasm_bindgen(constructor)]
          pub fn new() -> PolywrapClientWrapper;

          #[wasm_bindgen(method)]
          pub async fn invoke(this: &PolywrapClientWrapper, args: JsValue)
              -> JsValue;

          #[wasm_bindgen(method)]
          pub async fn invoke_method(
              this: &PolywrapClientWrapper,
              method: &str,
              args: JsValue,
          ) -> JsValue;

      }
      ```

      To call methods defined in the polywrap-substrate module will be simple as invoking the method of the client.

      ```rust
        pub async fn get_block_hash(
            client: &PolywrapClientWrapper,
            number: u32,
        ) -> Result<Option<String>, Error> {
            let args = json!({
                "url": SUBSTRATE_RPC_URL,
                "number": number,
            });
            let block_hash: Option<String> =
                client.invoke_method("blockHash", args).await?;
            Ok(block_hash)
        }
      ```


## Missing part
- The usage of `signer-provider` plugin in creating a signature of a message.
- This will be the last leg of the development process, the code for submitting extrinsic has already
    been prepared as we have decoupled the `composing`, `signing` and `submitting` the extrinsics into each functions to allow more flexibility.
- Development plan:
    1. use the `signer-provider` in the client directly as demonstrated in the code below.
    2. integrate the `signer-provider` call into the substrate `core-wrapper` module, so the user would be abstracted away from composing the extrinsic/payload themselves
        and only have to deal with calling `extrinsic` functions with their `pallet` and `call` name alongside with it's parameters.

    ```rust
        //********************************
        // -- This is the only part where private key is being used,
        // this should be delegated into the `signer-provider` plugin.
        //
        //  we use alice for now, for simplicity
        let signer: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
        // Signing just takes a series of bytes as input
        let signing_function = |payload: &[u8]| signer.sign(payload);
        // --end of private key usage------------
        //***************************************************

        let multi_signer = MultiSigner::from(signer.public());
        let signer_account = multi_signer.into_account();
        let nonce = get_nonce_for_account(client, &signer_account).await?;
        let signer_address: MultiAddress<AccountId32, ()> = signer_account.into();

        let extra = <..>;

        let call_index: [u8; 2] =
            pallet_call_index(client, "Balances", "transfer").await?;

        let dest: MultiAddress<AccountId32, ()> = MultiAddress::Id(to);
        let call: (
            [u8; 2],
            MultiAddress<AccountId32, ()>,
            Compact<u128>,
        ) = (call_index, dest, Compact(amount));

        let raw_payload = SignedPayload {
            call,
            extra,
            additional: (
                runtime_version.spec_version,
                runtime_version.transaction_version,
                genesis_hash,
                genesis_hash,
            ),
        };

        // Use the signing function above to generate a signature from the payload
        let signature = raw_payload.using_encoded(signing_function);
        let multi_signature = MultiSignature::from(signature);
    ```
    Then the signature can be included into an extrinsic which we can encode and submit into the
    chain using polywrap client.
    ```rust
        let extrinsic = UncheckedExtrinsicV4 {
            function: call,
            signature: Some((signer_address, multi_signature, extra)),
        };

        // raw bytes
        let encoded = extrinsic.hex_encode();
        // we send raw bytes into the polywrap client
        let tx_hash = author_submit_extrinsic(client, extrinsic).await?;
    ```
