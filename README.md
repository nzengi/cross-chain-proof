## Cross-Chain Merkle Proof - Blockchain Verification with Merkle Trees

Cross-Chain Merkle Proof is a Rust-based library designed to provide efficient cross-chain data verification using Merkle Trees. This project allows users to generate, verify, and manage proofs for cross-chain data integrity in blockchain environments, ensuring that data transferred between different blockchain networks remains secure and verifiable.

By using Merkle trees, our solution offers an efficient and scalable method for managing large amounts of data, while providing cryptographic guarantees of data integrity.

## Features

Merkle Tree Construction: Efficiently builds a binary Merkle Tree from a list of leaf data.
Proof Generation: Generates cryptographic proofs for verifying the inclusion of specific data in the Merkle Tree.
Cross-Chain Verification: Supports relay systems for submitting proofs to other blockchains for verification.
Rust-Based, Secure, and Optimized: Written in Rust to ensure performance and security, with a focus on memory safety and efficiency.

## Why Cross-Chain Merkle Proof?

In the rapidly expanding world of blockchain, the need for verifying data between different chains is growing. Our project leverages the power of Merkle trees to create a seamless solution for cross-chain data verification. By using this library, blockchain developers can ensure that data transferred between chains is:

Tamper-proof: Cryptographically secured using SHA-256 hashes.
Efficient: Merkle trees offer logarithmic scaling in terms of space and verification time.
Scalable: Suitable for managing large sets of data efficiently.

## Key Concepts

Merkle Tree: A binary tree where each leaf node represents a hash of a data block, and each non-leaf node represents the combined hash of its children.
Merkle Proof: A proof generated for a specific leaf in the tree that allows a verifier to confirm its inclusion in the root without needing to reveal all the data.
Cross-Chain Relay: A mechanism for submitting Merkle proofs between different blockchain networks to verify data integrity.


## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your machine
- Cargo (Rust package manager) installed

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/nzengi/cross-chain-proof.git
   cd cross-chain-merkle-proof
    ```
2. Build the project:

    ```bash
    cargo build
    ```

3. Run the project

    ```bash
    cargo run
    ```

This will start the API server at http://127.0.0.1:xxxx


## API Endpoints

1. Create Merkle Tree

URL: /merkle_tree
Method: POST
Description: Creates a Merkle Tree from the provided leaf nodes and returns the root hash.

Request Body:
```json
{
  "leaves": [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11]
  ]
}
```

Response:
```json
{
  "root_hash": [
    255, 243, 169, 188, 221, 55, 54, 61, 112, 60, 28, 79, 149, 18, 83, 54,
    134, 21, 120, 104, 240, 212, 241, 106, 15, 2, 208, 241, 218, 36, 249, 162
  ]
}
```

<<C>>





