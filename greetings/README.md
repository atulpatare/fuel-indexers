## Greetings

Simple greeter contract indexer

## Build

- Build the contract

```shell
# root dir contracts
forc build
```

- Run the tests

```shell
cargo test
```

- Run the interactions on the contract

```shell
# run a local node
fuel-core run --db-type in-memory

# run the function calls
cargo run -p interactions
```

- Build the indexer

```shell
# root dir indexer
carog build --release
```

- Run the indexer

```shell
# root dir fuel-indexer 
cargo build --release
./target/release/fuel-indexer --manifest ../fuel-indexers/greetings/indexer/manifest.yaml
```
