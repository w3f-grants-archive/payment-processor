## Payment Processor

Simulates a traditional payment processor which is used to process ISO-8583 messages from clients (i.e PoS terminals, ATMs, web payment gateways, etc.) and send them to the appropriate destination. You could think of it as a naive simulation of Stripe or PayPal.

It is part of the infrastracture for integrating ISO-8583 standard for Substrate.

### What it does?

#### Enpoints

Simple has one POST endpoint `/pos` which receives metadata of a plastic card and a transaction amount from the client and sends it to the [Oracle Gateway](../pcidss/README.md) for further processing.

#### PCIDSS Compliant Oracle

It maintains a constant websocket connection to the oracle gateway RPC and sends the ISO-8583 messages to it. When oracle is done with processing the message, it sends the response back to the payment processor which then sends it back to the client.

### How to run

It is a simple Express.js API, so you need to have `node` and `yarn/npm` installed on your machine. 

> **_NOTE:_** Was tested with `node` version `v20.5.1` and `yarn` version `1.22.10`.

```bash
yarn install

# start the dev server
yarn dev
```

> **_NOTE:_** You can set `PORT`, `ORACLE_RPC_URL` and `NODE_ENV` environment variables according to your needs.


### Testing

Unit tests cover the endpoints. You can run them with:

```bash
yarn test
```
