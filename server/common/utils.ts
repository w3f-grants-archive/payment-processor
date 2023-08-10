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
