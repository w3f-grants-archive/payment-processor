# Mock checkout and bank interface

This is a demo interface that uses the [PCIDSS Oracle Gateway](../pcidss/README.md) and [Payment Processor](../payment-processor/README.md) to simulate bank dashboard and demo checkout page.

## Overview

Main page is a simulation of a bank dashboard, where you can see basic details about the bank account and its transactions. To ease the testing, there is a button on the top right corner that allows you to switch between development accounts. You can use development accounts to simulate different scenarios.

## How to run

This is a React app and while running in development node, you need to run the [PCIDSS Oracle Gateway](../pcidss/README.md) and [Payment Processor](../payment-processor/README.md) in separate terminals. Please follow the instructions in the respective README files.

It also use the payment processor server running at `http://localhost:3000` as a proxy. If you want to change the port, you need to update the `package.json` file.

1. Run `yarn` to install dependencies
2. Run `yarn start` to start the interface
3. Open `http://localhost:3000` in your browser

## How to use

