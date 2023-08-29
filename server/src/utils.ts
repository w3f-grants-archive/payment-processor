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
 * Try to unwrap nullable or undefined value
 * @param value
 * @returns
 */
export function unwrap<T>(value: T | null | undefined): T {
  if (value === null || value === undefined) {
    throw new Error("Value is null or undefined");
  }
  return value;
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
