import { getInput, sendOutput } from './stylus';
export { mark_used } from './stylus'; // Required by Stylus
import { main } from '../app';
import { bytesToI32, i32ToBytes } from '../utils';

// Stylus entrypoint
export function user_entrypoint(len: i32): i32 {
  const input = getInput(len);
  if (!input) {
    return 1;
  }

  // Convert Uint8Array to i32 array for processing
  const inputArray: i32[] = [];
  for (let i = 0; i < input.length; i += 4) {
    const slice = input.slice(i, i + 4);
    inputArray.push(bytesToI32(slice));
  }

  // Calling the app
  const output = main(inputArray);

  // Convert i32 result back to Uint8Array
  sendOutput(i32ToBytes(output));
  return 0;
}
