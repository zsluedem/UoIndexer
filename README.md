# UoIndexer

OoIndexer is an [EIP-4337](https://eips.ethereum.org/EIPS/eip-4337) [UserOperation](https://github.com/eth-infinitism/account-abstraction/blob/develop/eip/EIPS/eip-4337.md#definitions) indexer.Currently, UoIndexer is still in **BETA**. Please use it at your own risk.

Currently, UoIndexer support 2 kinds of database storage.

1. [RocksDB](https://rocksdb.org/)
2. [MongoDB](https://www.mongodb.com/)

# Support chain

| ChainName     | chain id      |
| ------------- | ------------- |
| Ethereum      | 1  |
| Goerli  | 5  |


More to come later.

# Prerequisites

1. Clang
2. LLVM
3. [rust cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

# Install

```
cargo install --git https://github.com/zsluedem/UoIndexer
```

# How to use it

## Use RocksDB

```
uoindexer --rpc-url https://eth-mainnet.g.alchemy.com/v2/api-key --chain-id 1 rocks-db ./.local/rocksdb
```

## Use MongoDB

Supposed you have a mongo instance at localhost:27017

```
uoindexer --rpc-url https://eth-mainnet.g.alchemy.com/v2/api-key --chain-id 1 mongo-db mongo-db mongodb://root:example@localhost:27017/
```