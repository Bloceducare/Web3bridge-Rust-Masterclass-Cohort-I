export function i32ToBytes(value: i32): Uint8Array {
  const result = new Uint8Array(4);
  result[0] = (value >> 24) & 0xff; // Extract the first byte (most significant byte)
  result[1] = (value >> 16) & 0xff; // Extract the second byte
  result[2] = (value >> 8) & 0xff; // Extract the third byte
  result[3] = value & 0xff; // Extract the fourth byte (least significant byte)

  return result;
}

// TODO: Use bitwise operators to improve efficiency
export function bytesToI32(bytes: Uint8Array): i32 {
  if (bytes.length > 4) {
    throw new Error('Invalid Uint8Array length. Length should not be more than 4 bytes.');
  }

  const extendedBytes = new Uint8Array(4);
  for (let i = 0; i < 4; i++) {
    extendedBytes[i] = bytes.length > 3 - i ? bytes[bytes.length - (4 - i)] : 0x00;
  }

  let value: i32 = 0;

  for (let i = 0; i < 4; i++) {
    value += 0x100 ** i * extendedBytes[3 - i];
  }

  return value;
}
