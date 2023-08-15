#!/usr/bin/env node
import "@polkadot/wasm-crypto/initWasmAsm";
import "dotenv/config";
import { Router } from "express";
import { Accounts } from "./models";
import Server from "./server";
import { Routes } from "./types";

export default class MainRoute implements Routes {
  public path = "/";
  public router = Router();

  constructor() {
    this.initRoutes();
  }

  private initRoutes() {
    this.router.get(`${this.path}`, (req, res) => {
      res.send("Hello World");
    });

    this.router.post(`${this.path}pay`, (req, res) => {
      this.doPayment(req.body);
    });
  }

  // Try to pay the invoice
  async doPayment(body: any) {
    let { amount, cardNumber, cvv, expiry, recipientCardNumber } = body;

    // check if the sender has enough balance
    // if not, return error
    const sender: any = await Accounts.findOne({ where: { cardNumber } });
    if (sender.balance < amount) {
      return Promise.reject("Insufficient balance");
    }

    // check if the recipient is from this bank
    // if yes, do internal payment
    // if not, do external payment  
    const recipient = await Accounts.findOne({ where: { cardNumber } });

    return this.externalPayment(body);
  }

  // Try to pay for the transaction where the recipient is from external bank
  private externalPayment(body: any) {
    // try to pay the recipient
    // if successful, return the payment hash
    // if not, return the error
  }

  // Try to pay for the transaction where the recipient is from this bank
  private internalPayment(body: any) {
    // try to pay the recipient
    // if successful, return the payment hash
    // if not, return the error
  }
}

/**
 *
 * @returns Server instance
 */
async function main(): Promise<Server> {
  const app = await Server.init(
    [new MainRoute()],
    process.env.DATABASE_URL || "",
  );

  return Promise.resolve(app);
}

main()
  .then((app: Server) => {
    app.listen();
  })
  .catch((err) => {
    console.log("Encountered an error", err);
    process.exit(1);
  });
