# Verification

If one of the data blocks, such as Block3, is tampered with, the hash of Block3 will change. This change will propagate up the Merkle Tree, altering the non-leaf node hashes and ultimately changing the root hash. This is important for data integrity in blockchain because any change in the data will be immediately detectable by comparing the root hash, ensuring the data remains secure and unaltered.
