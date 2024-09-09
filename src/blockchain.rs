use std::collections::HashMap;
use crate::merkle_tree::Hash;

/// Structure representing Blockchain B
pub struct BlockchainB {
    pub known_roots: HashMap<u32, Hash>, // Stores block numbers and root hashes
}

impl BlockchainB {
    /// Verifies the proof using the known root hashes
    pub fn verify_proof(&self, _proof: Vec<Hash>) -> bool {
        // Placeholder verification logic
        println!("Verifying proof...");
        true
    }

    /// Adds a new root hash to the blockchain
    pub fn add_root(&mut self, block_number: u32, root_hash: Hash) {
        self.known_roots.insert(block_number, root_hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_root() {
        let mut blockchain = BlockchainB {
            known_roots: HashMap::new(),
        };
        let root_hash = vec![0u8; 32];
        blockchain.add_root(1, root_hash.clone());
        assert!(blockchain.known_roots.contains_key(&1));
    }

    #[test]
    fn test_verify_proof() {
        let blockchain = BlockchainB {
            known_roots: HashMap::new(),
        };
        let proof = vec![vec![0u8; 32]];
        let verified = blockchain.verify_proof(proof);
        assert!(verified);
    }
}

