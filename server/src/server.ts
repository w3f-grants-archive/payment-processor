import cors from "cors";
import express, { Router } from "express";
import helmet from "helmet";
import morgan from "morgan";
import { MCC, MTI, ProcessingCode, RequestBody } from "./types";
import { ensurePadded } from "./utils";

// @ts-ignore
import iso8583 from "iso_8583";

/**
 * Represents the server
 */
export default class Server {
  public app: express.Application;
  public env: string;
  public port: string | number;

  constructor() {
    this.app = express();
    this.env = process.env.NODE_ENV || "development";
    this.port = process.env.PORT || 3000;

    this.initMiddlewares();
  }

  // Starts the server
  public async start() {
    this.initRoutes();

    this.listen();
  }

  public listen() {
    this.app.listen(this.port, () => {
      console.log(`Server listening on port ${this.port}`);
    });
  }

  /// Initializes routes
  private initRoutes() {
    let router = Router();

    router.get("/", (req: express.Request, res: express.Response) => {
      res.send("Health check");
    });

    router.post("/pos", (req: express.Request, res: express.Response) =>
      this.doPayment(req, res)
    );

    this.app.use(router);
  }

  // POS implementation
  //
  // This function does the following:
  //
  // 1. Extract variables from the request
  // 2. Do some validation
  // 3. Form ISO-8583 message
  // 4. Send the message to the PCIDSS compliant oracle
  // 5. Wait for the response
  // 6. Send the response back to the client
  private async doPayment(req: express.Request, res: express.Response) {
    const data = this.formIsoData(req.body);

    const isopack = new iso8583(data);

    console.log("ISO8583 message", isopack.getBufferMessage());
    console.log("Is valid? ", isopack.validateMessage());
    console.log("Get MTI", isopack.getMti());
    console.log("Get Bmps Binary", isopack.getBmpsBinary());
  }

  // Forms a custom data to be passed to `ISO8583` pack
  private formIsoData(body: RequestBody): Record<string, string> {
    const { amount, cardNumber, cardExpiration, cvv }: RequestBody = body;

    const now = new Date();

    // Format is `hhmmss`
    const timeDate = [now.getHours(), now.getMinutes(), now.getSeconds()]
      .map((v) => ensurePadded(v.toString(), 2))
      .join("");

    // Format is `MMDD`
    const monthDay = [now.getMonth() + 1, now.getDate()]
      .map((v) => ensurePadded(v.toString(), 2))
      .join("");

    // Format is `MMDDhhmmss`
    const transmissionDate = `${monthDay}${timeDate}`;

    const track2 = `${cardNumber}D${cardExpiration}2011758928889`;
    console.log("Track-2 Data", track2);
    console.log("Transmission Date", transmissionDate);

    return {
      0: MTI.AuthorizationRequest,
      2: cardNumber,
      3: ProcessingCode.Purchase,
      4: ensurePadded(amount, 12), // Amount is 12 characters long, check the length of amount and pad it with `0` from the left
      7: transmissionDate,
      12: timeDate,
      13: monthDay,
      14: cardExpiration.replace("/", ""),
      18: MCC.ComputerNetworkServices,
      22: "051", // Point of Service Entry Mode
      23: "001", // Card Sequence Number
      26: "12", // Point of Service PIN Capture Code
      32: "423935", // Acquiring institution ID
      35: track2, // Track-2 Data
      41: "12345678", // Card Acceptor Terminal Identification
      42: "MOTITILL_000001", // Card Acceptor Identification Code
      43: "Dummy business name, Dummy City, 1200000", // Card Acceptor Name/Location
      49: "997", // Currency Code, Transaction, USD - 997, EUR - 978
      61: cvv,
      127: "0".repeat(50), // dummy 50 bytes, will be replaced in the future
    };
  }

  // Initializes middlewares
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
