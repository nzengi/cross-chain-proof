// Merkle Tree modülünü içe aktar
pub mod merkle_tree;

// Blockchain modülünü içe aktar
pub mod blockchain;

// Relay modülünü içe aktar
pub mod relay;

use crate::merkle_tree::MerkleTree;
use crate::blockchain::BlockchainB;
use crate::relay::Relay;
use std::collections::HashMap;

pub fn run_project() {
    // Merkle Tree: Create and generate root hash
    let leaves = vec![vec![0u8; 32], vec![1u8; 32], vec![2u8; 32]];
    let merkle_tree = MerkleTree::new(leaves);
    
    if let Some(root_hash) = merkle_tree.root_hash() {
        println!("Merkle root hash: {:?}", root_hash);
        
        // Blockchain B: Add the root hash
        let mut blockchain_b = BlockchainB {
            known_roots: HashMap::new(),
        };
        blockchain_b.add_root(1, root_hash.clone());

        // Relay: Submit proof to Blockchain B
        let proof = merkle_tree.generate_proof(&vec![0u8; 32]);
        Relay::submit_proof(proof, &mut blockchain_b);
    } else {
        println!("Failed to generate Merkle root.");
    }
}
