#!/usr/bin/env node

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
