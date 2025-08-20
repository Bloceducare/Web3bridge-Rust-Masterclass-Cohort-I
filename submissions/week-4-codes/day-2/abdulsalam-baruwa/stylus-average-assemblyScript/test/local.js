/*
 * This is a very simple local testing file. It only has support for read_args and write_result Stylus functions.
 */

// Imports
import fs from 'fs';

// Constants
const WASM_PATH = './build/release.wasm';

// Variables
let wasmModule;
let inputBytes;
let outputBytes;

// Main function
const main = () => {
  console.log('');
  console.log('-----------------');
  console.log('| Start program |');
  console.log('-----------------');
  console.log('');

  console.log('-----------------');
  const numbers = [1, 2, 3];
  console.log(`Input: [${numbers.join(', ')}]`);
  console.log('Calling program...');

  // Transform array of numbers into bytes (little-endian)
  const inputBytesBuffer = Buffer.alloc(numbers.length * 4);
  numbers.forEach((num, i) => {
    inputBytesBuffer.writeInt32LE(num, i * 4);
  });
  inputBytes = new Uint8Array(inputBytesBuffer.buffer);

  // Call wasm program
  const { user_entrypoint } = wasmModule.exports;
  user_entrypoint(inputBytes.byteLength);

  // Format result (expecting 4 bytes, little-endian)
  const avg = Buffer.from(outputBytes).readInt32LE(0);

  console.log(`Average: ${avg}`); // Should print 2
  console.log('-----------------');
  console.log('');
};

// Imports object
const wasmImports = {
  vm_hooks: {
    pay_for_memory_grow: () => { },

    read_args: (memoryPtr) => {
      const memory = new Uint8Array(wasmModule.exports.memory.buffer);
      const inputArray = new Uint8Array(inputBytes);

      for (let i = memoryPtr; i < memoryPtr + inputArray.length; i++) {
        memory[i] = inputArray[i - memoryPtr];
      }

      return;
    },

    write_result: (memoryPtr, length) => {
      const outputMemorySlice = wasmModule.exports.memory.buffer.slice(
        memoryPtr,
        memoryPtr + length,
      );
      outputBytes = new Uint8Array(outputMemorySlice);
    },
  },
};

////////////////
// Init point //
////////////////

// Arguments check

console.log('***********************');
console.log('* Stylus local tester *');
console.log('***********************');
console.log('');

// Getting arguments
const inputNumber = process.argv[2];

// Loading wasm file
console.log(`Loading WASM module in ${WASM_PATH}...`);
const wasmBuffer = fs.readFileSync(WASM_PATH);
WebAssembly.instantiate(wasmBuffer, wasmImports).then((wM) => {
  console.log('Module loaded!');
  wasmModule = wM.instance;
  main(inputNumber);
});
