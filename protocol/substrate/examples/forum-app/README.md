# A forum app example implemented on top of substrate blockchain using polywrap


This forum-app is an example web3 application showcasing the use of polywrap client specific to substrate node.
Additionally, the forum app is written in rust and compiled to wasm and is meant to be run on the browser.
Under the hood, it is using `wasm-bindgen` to manipulate DOM objects from rust and `sauron` web framework to simplify writing the application UI and components lifecycle.

## Prerequisite
Make sure the following are installed
- rust
- docker
- node
- yarn


## Background services
Run the custom substrate-node-template with forum pallet added
- `cd ../substrate-node-template && cargo run --release -- --dev`
Run the ipfs infrastructure
- `cd ../core-wrapper && yarn infra`

## Compile and run
```
yarn && yarn start
```
