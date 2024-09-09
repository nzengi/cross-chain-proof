use std::collections::HashMap;
use crate::merkle_tree::Hash;

/// Structure representing Blockchain B, a simplified blockchain simulation
/// that stores known Merkle root hashes and provides a proof verification mechanism.
pub struct BlockchainB {
    pub known_roots: HashMap<u32, Hash>, // Stores block numbers and corresponding Merkle root hashes.
}

impl BlockchainB {
    /// Verifies the proof using the known root hashes.
    ///
    /// This function is a placeholder for a more complex verification mechanism.
    /// In a real-world scenario, the proof would be checked against the Merkle root
    /// and the transaction data.
    ///
    /// # Arguments
    ///
    /// * `_proof` - A vector of hash values representing the proof to be verified.
    ///
    /// # Returns
    ///
    /// * `true` if the proof is verified successfully (dummy logic in this case),
    /// otherwise it would return `false`.
    pub fn verify_proof(&self, _proof: Vec<Hash>) -> bool {
        // Placeholder verification logic.
        println!("Verifying proof...");
        true
    }

    /// Adds a new Merkle root hash to the blockchain.
    ///
    /// This function simulates the process of adding a block with a specific
    /// Merkle root hash to the blockchain.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number associated with the Merkle root.
    /// * `root_hash` - The Merkle root hash to store in the blockchain.
    pub fn add_root(&mut self, block_number: u32, root_hash: Hash) {
        self.known_roots.insert(block_number, root_hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the functionality of adding a Merkle root hash to the blockchain.
    #[test]
    fn test_add_root() {
        // Initialize an empty blockchain.
        let mut blockchain = BlockchainB {
            known_roots: HashMap::new(),
        };
        // Example root hash (32-byte vector).
        let root_hash = vec![0u8; 32];
        // Add the root hash to the blockchain.
        blockchain.add_root(1, root_hash.clone());
        // Check if the block number 1 has the added root hash.
        assert!(blockchain.known_roots.contains_key(&1));
    }

    /// Test the functionality of verifying a proof.
    ///
    /// Note that the `verify_proof` function uses placeholder logic and always returns `true`.
    #[test]
    fn test_verify_proof() {
        // Initialize an empty blockchain.
        let blockchain = BlockchainB {
            known_roots: HashMap::new(),
        };
        // Example proof (32-byte vector of hashes).
        let proof = vec![vec![0u8; 32]];
        // Verify the proof (the result is always `true` for now).
        let verified = blockchain.verify_proof(proof);
        assert!(verified);
    }
}
