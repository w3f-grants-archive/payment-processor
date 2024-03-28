## Standalone integration tests

This crate contains standalone integration test that simulates an example of transfer that was triggered on-chain. For this
to work, we need the whole [infrastracture](https://github.com/subclone/payment-processor?tab=readme-ov-file#run-the-demo) for ISO-8583 message processing up and running.

Then, to run the tests:

```sh
cargo test -p oracle-e2e-tests
```
