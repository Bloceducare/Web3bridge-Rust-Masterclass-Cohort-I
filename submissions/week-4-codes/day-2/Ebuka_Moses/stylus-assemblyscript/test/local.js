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
  console.log(`Input: ${inputNumber}`);
  console.log('Calling program...');

  // Transform input number into bytes
  const inputBytesBuffer = Buffer.alloc(4);
  inputBytesBuffer.writeInt32BE(inputNumber, 0);
  inputBytes = new Uint8Array(inputBytesBuffer.buffer);

  // Call wasm program
  const { user_entrypoint } = wasmModule.exports;
  user_entrypoint(inputBytes.byteLength);

  // Format result
  const result = Buffer.from(outputBytes).readUIntBE(0, outputBytes.length);

  console.log(`Result: ${result}`);
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
if (process.argv.length <= 2) {
  console.log('Usage: yarn test:local 56');
  process.exit(1);
}

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
