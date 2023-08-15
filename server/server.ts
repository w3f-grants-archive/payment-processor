import cors from "cors";
import express from "express";
import helmet from "helmet";
import morgan from "morgan";
import { Sequelize } from "sequelize";
import { initDb } from "./models";
import { Routes } from "./types";

/**
 * Represents the server
 */
export default class Server {
  public app: express.Application;
  public env: string;
  public port: string | number;

  protected constructor(
    private readonly db: Sequelize,
    routes: Routes[]
  ) {
    this.app = express();
    this.env = process.env.NODE_ENV || "development";
    this.port = process.env.PORT || 3000;

    this.initMiddlewares();
    this.initRoutes(routes);
  }

  /**
   * Async initializer for API
   * @param options
   * @param routes
   * @returns
   */
  static async init(routes: Routes[], dbUrl: string): Promise<Server> {
    console.log("Initializing API...");

    const db = new Sequelize(dbUrl);
    
    // Authenticate DBs
    await db.authenticate();

    initDb(db);

    return new Server(db, routes);
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
