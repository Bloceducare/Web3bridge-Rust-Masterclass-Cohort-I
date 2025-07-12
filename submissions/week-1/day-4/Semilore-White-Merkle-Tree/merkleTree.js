// Import the crypto module for hashing
const crypto = require("crypto");

// Helper function to generate SHA-256 hash
function generateHash(data) {
  return crypto.createHash("sha256").update(data).digest("hex");
}

// Function to create a Merkle Tree
function createMerkleTree(blocks) {
  // Generate leaf nodes (hashes of the blocks)
  let leafNodes = blocks.map((block) => generateHash(block));

  // Build the tree
  let tree = [leafNodes];
  while (tree[0].length > 1) {
    let currentLevel = tree[0];
    let nextLevel = [];
    for (let i = 0; i < currentLevel.length; i += 2) {
      let left = currentLevel[i];
      let right = i + 1 < currentLevel.length ? currentLevel[i + 1] : left;
      nextLevel.push(generateHash(left + right));
    }
    tree.unshift(nextLevel);
  }
  return tree;
}

// Function to verify data integrity
function verifyIntegrity(originalTree, tamperedBlockIndex, tamperedBlock) {
  // Tamper with the block
  let tamperedLeafNodes = originalTree[originalTree.length - 1].slice();
  tamperedLeafNodes[tamperedBlockIndex] = generateHash(tamperedBlock);

  // Rebuilds the tree with the tampered block
  let tamperedTree = [tamperedLeafNodes];
  while (tamperedTree[0].length > 1) {
    let currentLevel = tamperedTree[0];
    let nextLevel = [];
    for (let i = 0; i < currentLevel.length; i += 2) {
      let left = currentLevel[i];
      let right = i + 1 < currentLevel.length ? currentLevel[i + 1] : left;
      nextLevel.push(generateHash(left + right));
    }
    tamperedTree.unshift(nextLevel);
  }

  return tamperedTree;
}

// Main function to demonstrate the Merkle Tree
function main() {
  const blocks = ["Block1", "Block2", "Block3", "Block4"];
  console.log("Original Blocks:", blocks);

  const merkleTree = createMerkleTree(blocks);
  console.log("Merkle Tree:", merkleTree);
  console.log("Root Hash:", merkleTree[0][0]);

  // Tamper with a block (change "Block3" to "TamperedBlock3")
  const tamperedBlockIndex = 2;
  const tamperedBlock = "TamperedBlock3";
  console.log(
    `\nTampering with Block ${tamperedBlockIndex + 1}: "${
      blocks[tamperedBlockIndex]
    }" -> "${tamperedBlock}"`
  );

  // Verify integrity
  const tamperedTree = verifyIntegrity(
    merkleTree,
    tamperedBlockIndex,
    tamperedBlock
  );
  console.log("Tampered Merkle Tree:", tamperedTree);
  console.log("New Root Hash:", tamperedTree[0][0]);

  // Check if the root hash changed
  if (merkleTree[0][0] === tamperedTree[0][0]) {
    console.log(
      "Integrity Check: Root hash did NOT change. Data integrity is compromised!"
    );
  } else {
    console.log(
      "Integrity Check: Root hash changed. Data integrity is maintained!"
    );
  }
}

main();
