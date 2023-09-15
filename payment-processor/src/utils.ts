import { ResponseCode } from "./types";

/**
 * Checks if the given value is hex string.
 * @param str
 * @returns boolean
 */
export function isHex(value: string | number): boolean {
  if (typeof value === "number") {
    return false;
  }
  return /^0x[0-9a-f]*$/i.test(value);
}

/**
 * Ensures that the given string is padded with zeros
 * @param value
 * @param length
 * @returns
 */
export function ensurePadded(value: string, length: number): string {
  if (value.length < length) {
    return value.padStart(length, "0");
  }
  return value;
}

/**
 * Response code to message
 * @param code
 * @returns string
 */
export function responseCodeToMessage(code: ResponseCode): string {
  switch (code) {
    case "00":
      return "Approved";
    case "05":
      return "Declined";
    case "51":
      return "Insufficient funds";
    case "54":
      return "Expired card";
    case "12":
      return "Invalid transaction";
    case "14":
      return "Invalid card";
    case "13":
      return "Invalid amount";
    default:
      return "Unknown error";
  }
}
