import { WsProvider } from "@polkadot/api";
import cors from "cors";
import express, { Router } from "express";
import helmet from "helmet";
import morgan from "morgan";
import { MCC, MTI, ProcessingCode, RequestBody } from "./types";
import { ensurePadded, responseCodeToMessage } from "./utils";
// @ts-ignore
import iso8583 from "iso_8583";

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
      process.env.ORACLE_RPC_URL || "ws://127.0.0.1:3030"
    );

    this.initMiddlewares();
  }

  // Starts the server
  public async start() {
    await this.oracle_rpc.connect();
    console.log(
      "Connected to oracle at",
      process.env.ORACLE_RPC_URL || "ws://127.0.0.1:3030"
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

    console.log(
      "Full message",
      isopack.getIsoJSON(isopack.getBufferMessage(), {
        lenEncoding: "hex",
        bitmapEncoding: "hex",
        secondaryBitmap: true,
      })
    );

    try {
      let msgResponse = await this.oracle_rpc.send("pcidss_submit_iso8583", [
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
    });
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

    const track2 = `${cardNumber}D${cardExpiration}C${cvv}`;

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
      32: "123456", // Acquiring institution ID, hard coded, for now
      35: track2, // Track-2 Data
      41: "12345678", // Card Acceptor Terminal Identification
      42: "ABCDEFGH_000001", // Card Acceptor Identification Code
      43: "Dummy business name, Dummy City, 1200000", // Card Acceptor Name/Location
      49: "997", // Currency Code, Transaction, USD - 997, EUR - 978
      126: "0".repeat(100), // dummy 100 bytes, will be replaced in the future
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
