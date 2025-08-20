import { i32ToBytes, bytesToI32 } from './utils';


/* Returns the minimum even prime number, which is always 2
 * This is a simplified version of the original sieve algorithm
 */

function getMinEvenPrime(_number: i32): i32 {
  return 2; // The only even prime number
}

/**
 * Main function of your contract
 * @dev Receives the input of bytes in Uint8Array. Result must also be sent in bytes wrapped in Uint8Array
 *
 * @param input bytes in Uint8Array
 * @returns bytes in Uint8Array
 */
export const main = (input: Uint8Array): Uint8Array => {
  const maxNumber = bytesToI32(input);
  // const maxPrime = getMaxPrimeUpTo(maxNumber);
  // return i32ToBytes(maxPrime);
  const minEvenPrime = getMinEvenPrime(maxNumber);
  return i32ToBytes(minEvenPrime);
};
