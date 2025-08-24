/*
 * Modify the constants in the "Constants" section with the address of your contract
 */

// Imports
import { ethers } from 'ethers';

// Constants
const CONTRACT_ADDRESS = '0xf5ffd11a55afd39377411ab9856474d2a7cb697e';
const RPC_URL = 'http://localhost:8547/';

// Initial setup
const provider = new ethers.JsonRpcProvider(RPC_URL);

// Main function
const main = async (inputNumber) => {
  console.log('');
  console.log('------------------');
  console.log('| Start contract |');
  console.log('------------------');
  console.log('');

  console.log('-----------------');
  console.log(`Input: ${inputNumber}`);
  console.log('Calling contract...');

  const resultBytes = await provider.call({
    to: CONTRACT_ADDRESS,
    data: ethers.toBeHex(inputNumber),
  });
  const result = parseInt(resultBytes, 16);

  console.log(`Result: ${result}`);
  console.log('-----------------');
  console.log('');
};

////////////////
// Init point //
////////////////

// Arguments check
if (process.argv.length <= 2) {
  console.log('Usage: yarn test:onchain 56');
  process.exit(1);
}

console.log('*************************');
console.log('* Stylus onchain tester *');
console.log('*************************');
console.log('');

// Getting arguments
const inputNumber = process.argv[2];

main(inputNumber)
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
