import { getInput, sendOutput } from './stylus';
export { mark_used } from './stylus'; // Required by Stylus
import { main } from '../app';

// Stylus entrypoint
export function user_entrypoint(len: i32): i32 {
  const input = getInput(len);
  if (!input) {
    return 1;
  }

  // Calculate average
  const output = main(input);

  sendOutput(output);
  return 0;
}
