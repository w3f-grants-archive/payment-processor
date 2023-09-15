# ISO-8553 Infrastracture

This repository contains the infrastructure for ISO-8553 implementation for Substrate based chains. It contains of parts that are responsible for processing ISO-8553 messages, maintaining offchain ledger. In the next milestone, integration with Substrate chain will be added.

## Notes

Some important notes about the project:

- It is a PoC implementation, so there are many places where we cut corners and some things are hard coded.
- ISO-8583:1987 standard is used.
- Substrate chain integration is not implemented yet.
- Unit tests are more like integration tests (somewhat similar to Substrate).

## Contents

Contains three key parts for ISO-8553 message processing:

1. Demo merchant application
2. Server application for processing requests from merchant application
3. PCIDSS compliant gateway simulator

### Demo Merchant Application

React application with mock bank interface and a demo checkout page.

### Payment Processor Server

NodeJs server that is responsible for processing requests from merchant application and sending them to the gateway simulator.

### PCIDSS Compliant Oracle Gateway

JSON-RPC based message consumer that maintains offchain ledger and a processor for ISO-8553 messages. In the future will have Substrate chain watcher service integrated.

## References

- [ISO-8583 Research](https://github.com/adit313/ISO8583-Blockchain-Integration-Plan) by Adit Patel
- [ISO-8583 Specification](https://www.iso.org/standard/15870.html) ISO-8583:1987 is used
