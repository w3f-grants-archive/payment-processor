import { Options } from "sequelize";

/**
 * The options to pass to script.
 */
export interface CliOptions {
  sync: boolean;
  subscribe: boolean;
  chain: string;
  wsUrl: string;
  dbUrl: string;
  resultDbUrl: string;
  startBlock: string;
  batch: string;
}

/**
 * Options to pass to API
 */
export interface ApiOptions {
  dbUrl: string;
  dbOptions: Options;
  queryDbUrl: string;
  queryDbOptions: Options;
  sync: boolean;
  wsUrl: string;
}

/**
 * Options for the database.
 */
export interface QueryOptions {
  dbUrl: string;
  resultDbUrl: string;
  dbOptions: Options;
  wsUrl: string;
  startBlock: number;
  batch: number;
  subscribe: boolean;
  force: boolean; // forces to start syncing from the `startBlock`
  sync?: boolean; // for db
}

/**
 * Generice interface for an event.
 */
export interface BaseEvent {
  id: string;
  blockNumber: number;
  method: string;
  section: string;
  index: number;
  phaseType: string;
  phaseIndex: number;
  args: (number | string)[];
}

/**
 * Base interface for a dispatchable call.
 */
export interface BaseCall {
  blockNumber: number;
  args: any;
  extrinsicId: string;
  method: string;
  section: string;
  index: number;
  hash: string;
}

/**
 * Represents a full block that contains all the data that's interesting for us.
 */
export interface FullBlockType {
  blockNumber: number;
  blockHash: string;
  parentHash: string;
  events: BaseEvent[];
  calls: BaseCall[];
  processed: boolean | null;
}

/**
 * Block data in a more interesting format.
 * @param blockNumber - Block number.
 * @param blockHash - Block hash.
 * @param extrinsicId - id of the extrinsic.
 * @param extrinsicMethod - method of the extrinsic.
 * @param dispatchableCalls - dispatchable calls from the extrinsic.
 * @param events - events related to the extrinsic.
 */
export interface BlockData {
  blockNumber: number;
  timestamp: number;
  extrinsicId: string;
  extrinsicIndex: number;
  extrinsicSection: string;
  extrinsicMethod: string;
  dispatchableCalls: BaseCall[];
  events: BaseEvent[];
  systemEvents: BaseEvent[];
}
