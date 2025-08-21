import { i32ToBytes, bytesToI32 } from './utils';

// Unsigned square root
function usqrt(n: u32): u32 {
  let x = n;
  let y = (x + 1) >> 1;
  while (y < x) {
    x = y;
    y = (x + n / x) >> 1;
  }
  return x;
}

/**
 * Returns the max prime below or equal a given "number" using the Sieve of Eratosthenes algorithm
 * (Based on t-katsumura's implementation: https://github.com/t-katsumura/webassembly-examples-eratosthenes)
 * (Sieve of Eratosthenes explanation: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
 */
function getMaxPrimeUpTo(number: i32): i32 {
  // 0 here is meant to be interpreted as an error
  if (number < 2) {
    return 0;
  }

  // If 2 is passed, return 2
  if (number == 2) {
    return 2;
  }

  // Length of the sieve array
  const length = (number - 1) / 2;

  // Square root (max)
  const maxNumberToCheck = usqrt(number);

  // Sieve array (starting from 3, without multiples of 2) [3, 5, 7, 9, ...]
  const sieve = new StaticArray<bool>(length).fill(true);

  // Coordinates of the Sieve
  let x: u32;
  let y: u32;

  for (let i = 0; i < length; i++) {
    // Next number to check multiples for
    x = 2 * (i + 1) + 1;

    // No need to check multiples for numbers that are greater than
    // the square root of the upper limit
    if (x > maxNumberToCheck) {
      break;
    }

    // Checking multiples of x
    for (let j = i + 1; j < length; j++) {
      if (!unchecked(sieve[j])) {
        continue;
      }

      // Next multiple candidate
      y = 2 * (j + 1) + 1;

      // Candidate is multiple of x (not prime then)
      if (y % x == 0) {
        unchecked((sieve[j] = false));
      }
    }
  }

  // Get the highest prime number
  let max_val: u32 = 2;
  for (let i = length - 1; i >= 0; i--) {
    if (unchecked(sieve[i])) {
      max_val = 2 * (i + 1) + 1;
      break;
    }
  }
  return max_val;
}

/**
 * Main function of your contract
 * @dev Receives the input of bytes in Uint8Array. Result must also be sent in bytes wrapped in Uint8Array
 *
 * @param input bytes in Uint8Array
 * @returns bytes in Uint8Array
 */
// export const main = (input: Uint8Array): Uint8Array => {
//   const maxNumber = bytesToI32(input);
//   const maxPrime = getMaxPrimeUpTo(maxNumber);
//   return i32ToBytes(maxPrime);
// };

/**
 * Returns the minimum even prime number
 * Since 2 is the only even prime number, this function always returns 2
 */
function getMinEvenPrime(number: i32): i32 {
  // The only even prime number is 2
  // If the input is less than 2, return 0 as error (following original pattern)
  if (number < 2) {
    return 0;
  }
  
  // For any input >= 2, the minimum even prime is always 2
  return 2;
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
