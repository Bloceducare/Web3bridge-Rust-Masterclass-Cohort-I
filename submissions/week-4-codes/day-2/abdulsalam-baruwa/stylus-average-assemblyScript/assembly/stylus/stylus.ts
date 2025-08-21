import { pay_for_memory_grow, read_args, write_result } from './hostio';

// Fallback functions
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function abort(message: usize, fileName: usize, line: u32, column: u32): void {
  return;
}

// Helper functions
export function mark_used(): void {
  pay_for_memory_grow(0);
}

export function getInput(len: i32): Uint8Array | null {
  const input = new Uint8Array(len);
  read_args(input.dataStart as u32);
  return input;
}

export function sendOutput(output: Uint8Array): void {
  write_result(output.dataStart as u32, output.byteLength);
}
