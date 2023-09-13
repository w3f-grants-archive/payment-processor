# ISO-8553 Infrastracture

This repository contains the infrastructure for ISO-8553 implementation for Substrate based chains. It contains of parts that are responsible 

## Contents

Contains three key parts for ISO-8553 message processing:

1. Demo merchant application
2. Server application for processing requests from merchant application
3. PCIDSS compliant gateway simulator

### Demo merchant application

Web application that is mainly a checkout page for a merchant.

### Server application

NodeJs server that is responsible for processing requests from merchant application and sending them to the gateway simulator.

### PCIDSS compliant gateway simulator

RabbitMQ based message broker combined with a Rust service that is for syncing with Substrate chain.
