// Import the Merkle Tree module
pub mod merkle_tree;

// Import the Blockchain module
pub mod blockchain;

// Import the Relay module
pub mod relay;

use crate::merkle_tree::MerkleTree;
use crate::blockchain::BlockchainB;
use crate::relay::Relay;
use std::collections::HashMap;

/// Run the cross-chain Merkle proof project.
///
/// This function initializes a Merkle Tree from some predefined data, generates
/// a root hash, adds that hash to a blockchain, and submits a proof to verify 
/// the inclusion of a specific leaf node. It demonstrates the basic functionality
/// of the library, including Merkle Tree creation, proof generation, and cross-chain
/// verification through a relay.
///
/// # Example
///
/// ```rust
/// use cross_chain_merkle_proof::run_project;
///
/// fn main() {
///     run_project();
/// }
/// ```
pub fn run_project() {
    // Merkle Tree: Create a new Merkle Tree with three leaf nodes.
    // Each leaf is represented by a 32-byte vector of data.
    let leaves = vec![vec![0u8; 32], vec![1u8; 32], vec![2u8; 32]];
    let merkle_tree = MerkleTree::new(leaves);

    // Check if the Merkle Tree has a valid root hash.
    if let Some(root_hash) = merkle_tree.root_hash() {
        // Print the root hash of the Merkle Tree.
        println!("Merkle root hash: {:?}", root_hash);
        
        // Blockchain B: Initialize a blockchain and add the Merkle root hash.
        let mut blockchain_b = BlockchainB {
            known_roots: HashMap::new(),
        };
        blockchain_b.add_root(1, root_hash.clone());

        // Relay: Generate a proof for the first leaf node (vec![0u8; 32])
        // and submit it to Blockchain B for verification.
        let proof = merkle_tree.generate_proof(&vec![0u8; 32]);
        Relay::submit_proof(proof, &mut blockchain_b);
    } else {
        // If the Merkle Tree root is invalid, print an error message.
        println!("Failed to generate Merkle root.");
    }
}
