#!/usr/bin/env node

// each chain is different in what they expose runtime-wise
// enable it on a static basis (westend being closest to polkadot)
import "@polkadot/api-augment/polkadot";
import Server from "./server";

/**
 *
 * @returns Server instance
 */
async function main() {
  const server = new Server();
  server.start();
}

main()
  .then(() => {
    console.log("Server started");
  })
  .catch((err) => {
    console.log("Encountered an error", err);
    process.exit(1);
  });
