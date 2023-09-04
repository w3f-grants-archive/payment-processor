## PCIDSS Compliant Oracle Gateway

PCIDSS Compliant Oracle Gateway is a message broker service, which is used to process ISO-8583 messages from payment processors and send them to the appropriate destination. It currently mocks a traditional bank which has issued some cards and is currently used for implementing a PoC implementation of ISO-8583 on a Substrate based blockchain.

### How to run

You need to have `rust` and `cargo` installed on your machine. 

```bash
cargo build -p pcidss-oracle
```

## Documentation

More on proposed architecture:

![Implementation plan](https://github.com/dastansam/Grants-Program/assets/88332432/8b832448-9095-4846-95ea-ccaebe5e52a5)
