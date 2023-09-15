import { WsProvider } from "@polkadot/api";
import cors from "cors";
import express, { Router } from "express";
import helmet from "helmet";
import morgan from "morgan";
import { MCC, MTI, ProcessingCode, RequestBody } from "./types";
import { ensurePadded, responseCodeToMessage } from "./utils";
// @ts-ignore
import iso8583 from "iso_8583";

// Custom format for `126` field
let CUSTOM_FORMATS = {
  "126": {
    ContentType: "ans",
    Label: "Private data",
    LenType: "llvar",
    MaxLen: 100,
  },
};

/**
 * Represents the server
 */
export default class Server {
  public app: express.Application;
  public oracle_rpc: WsProvider;
  public env: string;
  public port: string | number;

  constructor() {
    this.app = express();
    this.env = process.env.NODE_ENV || "development";
    this.port = process.env.PORT || 3000;
    this.oracle_rpc = new WsProvider(
      process.env.ORACLE_RPC_URL || "ws://0.0.0.0:3030"
    );

    this.initMiddlewares();
  }

  // Starts the server
  public async start() {
    await this.oracle_rpc.connect();
    console.log(
      "Connected to oracle at",
      process.env.ORACLE_RPC_URL || "ws://0.0.0.0:3030"
    );

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

    router.post("/pos", async (req: express.Request, res: express.Response) =>
      this.submitIso8583(req, res)
    );

    router.post(
      "/reverse",
      async (req: express.Request, res: express.Response) => {
        this.submitIso8583(req, res);
      }
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
  private async submitIso8583(req: express.Request, res: express.Response) {
    const data = this.formIsoData(req.body);
    const isopack = new iso8583(data, CUSTOM_FORMATS);

    try {
      let msgResponse = await this.oracle_rpc.send("pcidss_submit_iso8583", [
        // slicing the first two bytes, because they are the length of the message
        // RPC doesn't expect it
        Array.from(isopack.getBufferMessage().slice(2)),
      ]);

      await this.processResponse(msgResponse, res);
    } catch {
      res.status(500).json({
        status: false,
        message: "Internal server error",
      });
    }
  }

  // Processes the response from the oracle
  private async processResponse(isoMsg: any[], res: express.Response) {
    // convert length of message to two bytes u16
    let len = isoMsg.length;
    let lenBytes: any[] = [];
    lenBytes[0] = len >> 8;
    lenBytes[1] = len & 0xff;

    let isopack = new iso8583();
    let msg: any = isopack.getIsoJSON(Buffer.from(lenBytes.concat(isoMsg)), {
      lenEncoding: "hex",
      bitmapEncoding: "hex",
      secondaryBitmap: true,
    });

    console.log("Response from oracle", msg);

    const responseCode = msg["39"];

    let message = responseCodeToMessage(responseCode);

    res.status(200).json({
      status: responseCode === "00",
      message,
      result: msg["126"],
    });
  }

  // Forms a custom data to be passed to `ISO8583` pack
  private formIsoData(body: RequestBody): Record<string, string> {
    const isoNow = new Date();
    const now = isoNow.toISOString();

    // Format is `hhmmss`
    const timeDate = [
      now.slice(11, 13),
      now.slice(14, 16),
      now.slice(17, 19),
    ].join("");

    // Format is `MMDD`
    const monthDay = [now.slice(5, 7), now.slice(8, 10)].join("");
    const { amount, cardNumber, cardExpiration, cvv, txHash }: RequestBody =
      body;

    let isReversal = txHash !== null;

    // Format is `MMDDhhmmss`
    const transmissionDate = `${monthDay}${timeDate}`;

    const track2 = `${cardNumber}D${cardExpiration.replace("/", "")}C${cvv}`;

    return {
      0: isReversal ? MTI.ReversalRequest : MTI.AuthorizationRequest,
      2: cardNumber,
      3: ProcessingCode.Purchase,
      4: ensurePadded(amount, 12), // Amount is 12 characters long, check the length of amount and pad it with `0` from the left
      7: transmissionDate,
      12: timeDate,
      13: monthDay,
      14: cardExpiration.replace("/", ""),
      18: MCC.ComputerNetworkServices,
      32: "123456", // Acquiring institution ID, hard coded, for now
      35: track2, // Track-2 Data
      41: "12345678", // Card Acceptor Terminal Identification
      42: "ABCDEFGH_000001", // Card Acceptor Identification Code
      43: "Dummy business name, Dummy City, 1200000", // Card Acceptor Name/Location
      49: "997", // Currency Code, Transaction, USD - 997, EUR - 978
      126: txHash ? txHash : "0".repeat(99), // dummy 100 bytes, will be replaced in the future
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
