## PCIDSS Compliant Oracle Gateway

PCIDSS Compliant Oracle Gateway is an RPC service, which is used to process ISO-8583 messages from payment processors. It currently mocks a traditional bank which has issued some cards and is currently used for implementing a PoC implementation of ISO-8583 on a Substrate based blockchain.

### What it does?

Oracle service implements an RPC API for processing incoming ISO-8583 messages. It also implements `MsgProcessor` trait from `iso_8583rs`. 

In the future watcher service will be added, which will be used to subscribe to the Substrate chain for further integration of ISO-8583 standard.

### How to run

**Pre-requisites**

- Rust toolchain (tested with version 1.72.0)
- Postgres database (tested with version 14.9)

To run the oracle:

```bash
cargo run -p pcidss-oracle
```

To build the binary for release and run it:

```bash
cargo build -p pcidss-oracle -r

./target/release/pcidss-oracle
```

#### CLI

Oracle service accepts the following arguments (which can be seen by running `pcidss-oracle --help`):

```bash
Usage: pcidss-oracle [OPTIONS]

Options:
      --database-host <DATABASE_HOST>
          Path to the Postgres database [default: localhost]
      --database-port <DATABASE_PORT>
          Port of the Postgres database [default: 5432]
      --database-user <DATABASE_USER>
          Username of the Postgres database [default: postgres]
      --database-name <DATABASE_NAME>
          Name of the Postgres database [default: postgres]
      --chain-endpoint <CHAIN_ENDPOINT>
          Substrate chain websocket endpoint [default: ws://localhost:9944]
      --iso8583-spec <ISO8583_SPEC>
          ISO-8583 specification file [default: spec.yaml]
      --rpc-port <RPC_PORT>
          RPC port [default: 3030]
  -h, --help
          Print help
```

#### Testing

Oracle service has unit tests for the ISO-8583 message processing logic. You can run them with:

```bash
cargo test -p pcidss-oracle
```

#### Testing with payment processor

You can test the oracle service with the [payment processor](../payment-processor/README.md). For this, you will need to run this script that creates mock plastic cards.

```bash
make create-mock-cards
```

Then you can run the payment processor:

```bash
cd ../payment-processor
yarn run dev-server
```

## Documentation

More on proposed architecture:

![Implementation plan](https://github.com/dastansam/Grants-Program/assets/88332432/8b832448-9095-4846-95ea-ccaebe5e52a5)
