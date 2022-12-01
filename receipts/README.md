## Receipts

Indexer to index types from a receipt

## Pre-requisite

- Get the fuel indexer

```shell
git clone https://github.com/FuelLabs/fuel-indexer.git
cd fuel-indexer
cargo build --release
```

## Execution

- Build

```shell
cargo build --release
```

- Run

```shell
# root from fuel-indexer package
cargo build --release
./target/release/fuel-indexer --manifest ../fuel-indexers/receipts/manifest.yaml
```