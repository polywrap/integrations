# Substrate core wrapper

## Prerequisite
- Install a recent version of node
    `nvm install & nvm use`

## Building and testing
1. Download the needed dependencies
```shell
yarn
```

2. Build
```shell
yarn codegen && yarn build
```

3. Run the test
```shell
yarn test
```

## Deploying the wrapper
```
yarn deploy
```


Take note of the hash, this will be use to set the URI when using the module
```shell
Successfully executed stage 'ipfs_deploy'. Result: 'wrap://ipfs/QmUShkhii5JUM9t3RnZtS2kTUqReSjNVHQ9NaMzEMazqJ9'
Done in 3.80s.
```
