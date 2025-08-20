import { i32ToBytes, bytesToI32 } from './utils';

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function getMinEvenPrime(_n: i32): i32 {
  return 2; // the only even prime
}
/**
 * Main function of your contract
 * @dev Receives the input of bytes in Uint8Array. Result must also be sent in bytes wrapped in Uint8Array
 *
 * @param input bytes in Uint8Array
 * @returns bytes in Uint8Array
 */
export const main = (input: Uint8Array): Uint8Array => {
  const inputNumber = bytesToI32(input);
  const minEvenPrime = getMinEvenPrime(inputNumber);
  return i32ToBytes(minEvenPrime);
};
