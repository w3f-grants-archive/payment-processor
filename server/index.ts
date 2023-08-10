#!/usr/bin/env node
import "@polkadot/wasm-crypto/initWasmAsm";
import "dotenv/config";
import Server from "./server";
import IndexRoute from "./routes/index";
import EraStatsRoute from "./routes/era_stats";

/**
 *
 * @returns Server instance
 */
async function main(): Promise<Server> {
  const app = await Server.init(
    {
      dbUrl: process.env.DATABASE_URL_POLKADOT || "",
      dbOptions: {
        dialect: "postgres",
        logging: false,
        pool: {
          max: 10,
          min: 0,
          acquire: 30000,
          idle: 10000,
        },
      },
      queryDbOptions: {
        dialect: "postgres",
        logging: false,
        pool: {
          max: 10,
          min: 0,
          acquire: 30000,
          idle: 10000,
        },
      },
      sync: true,
      wsUrl: process.env.WS_URL || "wss://rpc.polkadot.io",
    },
    [new IndexRoute(), new EraStatsRoute()]
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
