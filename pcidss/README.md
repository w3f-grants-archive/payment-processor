# PCIDSS Compliant Payment Gateway

It's a worker that receives a payment request and returns a payment response.

If the payment is approved, it will settle the transaction in the ledger (in the future it will be also reflected on-chain, when blockchain part is implemented).

## Build

```bash
$ make build
```

## Run

```bash
$ make run
```

## Test

```bash

$ make test
```
