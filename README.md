# ISO-8553 Infrastracture

This repository contains the infrastructure for ISO-8553 integration PoC for Substrate based chains. It contains of parts that are responsible for processing ISO-8553 messages, maintaining offchain ledger and synchronizing it with Substrate chain. These components, provides a mock environment for full cycle of ISO-8583 messages.

This is the high-level overview of the infrastracture:

![iso-8583-overview](https://github.com/subclone/payment-processor/assets/88332432/939a8e5c-0b2e-4735-b0f4-003726008248)


## Notes

There are some important assumptions and notes you should be aware of before testing this PoC:

- that it is a PoC and should not be used in production
- chain relies on the trusted oracle and payment processor server and serves as the settlement/extension layer of the existing financial system
- single source of truth is the offchain ledger, for the sake of simplicity. In the future, it would be possible to implement a more complex system where the on-chain balances are more important.
- oracles are in a semi-trusted environment, i.e. they are trusted to sign transactions, but not to decide on the validity of the transactions. This is done by the payment processor.
- the payment processor is a trusted entity that is responsible for the finality of the transactions. It is PCIDSS compliant and is responsible for the security of the funds.
- ISO-8583:1987 standard is used.

## Run the demo

For demonstration purposes, `docker-compose` configuration is provided. It will start the following key services for ISO-8553 message processing:

- [Demo merchant application](./interface/README.md)
- [Payment Processor Server](./payment-processor/README.md)
- PCIDSS compliant [oracle](./pcidss/README.md)
- Postgres database

To start the demo, first pull the images:

```bash
docker-compose pull
```

Then start the services:

```bash
docker-compose up
```

You will be able to access the demo merchant application at `http://0.0.0.0:3002`.

And assuming you are running the Substrate chain, you can access the explorer [here](https://polkadot.js.org/apps/?rpc=ws://localhost:9944#/explorer).

## Demo flow

Main demo walkthrough is located [here](./DEMO.md). It contains comprehensive information about the demo flow, interactions between the components and the user facing side of the infrastracture. It also provides step by step guide on how to test main features of the milestone.

### Demo Merchant Application

React application with mock bank interface, on-chain address registration and a demo checkout page.

### Payment Processor Server

NodeJs server that is responsible for processing requests from merchant application and sending them to the gateway simulator.

### PCIDSS Compliant Oracle Gateway

JSON-RPC based message consumer that maintains offchain ledger and a processor for ISO-8553 messages. It also has a watcher services that is subscribed to Substrate chain to look for intents and also settle finality of off-chain ledger transactions.

## Tests, clippy, fmt and coverage

```bash
# Run check
cargo check --all-features
# Run all tests: unit, semi-integration and doc tests
cargo test --workspace --all-features --exclude oracle-e2e-tests
# Run clippy
cargo clippy --workspace --all-targets --all-features
# Run fmt
cargo +nightly fmt --all --check
# Run code coverage
cargo tarpaulin --workspace --all-features --exclude oracle-e2e-tests
```

For running `e2e-tests`, please, refer to `e2e-tests` [README](./pcidss/e2e-tests/README.md).

## References

- [ISO-8583 Research](https://github.com/adit313/ISO8583-Blockchain-Integration-Plan) by Adit Patel
- [ISO-8583 Specification](https://www.iso.org/standard/15870.html) ISO-8583:1987 is used
