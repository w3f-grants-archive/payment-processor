import express, { query } from "express";
import helmet from "helmet";
import { Sequelize } from "sequelize";
import morgan from "morgan";
import cors from "cors";
import { Routes } from "./interfaces";
import { ApiOptions } from "../common/types";
import { initResultDb } from "../common/db";
import initIndexerDb from "@open-web3/indexer/models/index";
import { ApiPromise, WsProvider } from "@polkadot/api";

export default class Server {
  public app: express.Application;
  public env: string;
  public port: string | number;

  protected constructor(
    private readonly db: Sequelize,
    private readonly resultDb: Sequelize,
    private readonly substrateApi: ApiPromise,
    routes: Routes[]
  ) {
    this.app = express();
    this.env = process.env.NODE_ENV || "development";
    this.port = process.env.PORT || 3000;

    this.initMiddlewares();
    this.initRoutes(routes);
    // this.initErrorHandlers();
  }

  /**
   * Async initializer for API
   * @param options
   * @param routes
   * @returns
   */
  static async init(options: ApiOptions, routes: Routes[]): Promise<Server> {
    console.log("Initializing API...");

    const { dbUrl, dbOptions, queryDbUrl, queryDbOptions, sync, wsUrl } =
      options;

    const db = new Sequelize(dbUrl, dbOptions);
    const queryDb = new Sequelize(queryDbUrl, queryDbOptions);

    // Authenticate DBs
    await Promise.all([await db.authenticate(), await queryDb.authenticate()]);

    initIndexerDb(db);
    initResultDb(db);

    if (sync) {
      await Promise.all([await db.sync(), await queryDb.sync()]);
    }

    const wsProvider = new WsProvider(wsUrl);

    const api = await ApiPromise.create({
      provider: wsProvider,
    });

    return new Server(db, queryDb, api, routes);
  }

  public listen() {
    this.app.listen(this.port, () => {
      console.log(`Server listening on port ${this.port}`);
    });
  }

  public getServer(): express.Application {
    return this.app;
  }

  private initRoutes(routes: Routes[] = []) {
    routes.forEach((route) => {
      this.app.use("/", route.router);
    });
  }

  private initMiddlewares() {
    this.app.use(
      morgan("combined", {
        stream: {
          write: (message) =>
            console.log(message.substring(0, message.lastIndexOf("\n"))),
        },
      })
    );
    this.app.use(cors());
    this.app.use(helmet());
    this.app.use(express.json());
    this.app.use(express.urlencoded({ extended: true }));
  }
}
