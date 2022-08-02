# TODO:
- [ ] Make an example app, showcasing multiple functionalities of the substrate chain
    - [X] balance transfer
    - [X] storage api
        - [X] storage_value
        - [X] storage_maps
    - [X] pallet constants
- [X] Deploy and use the substrate-polywrapper to ipfs
    - [ ] Make use of ENS domain to make the deployment automatic
        - Currently, there is a need to copy paste the deployed ipfs hash
- [ ] Make use of signer + provider
- [X] Restructure extrinsic api, such that it talks to a service which the user signs the payload and give its public key
        - execute_extrinsic
        - balance_transfer
    use only of the Public key instead of the the Pair, as it would be violating proper security practice.
    The payload/message should be signed by a different api calling on the signer and provider plugin of polywrap
- [ ] Make the module API take error into considerations
    - Right now, we are only returning `Option` instead of `Result` with proper error codes
- [ ] Add documentation for each of the api function
- [ ] Add rust test in rust code
