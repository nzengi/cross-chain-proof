use crate::blockchain::BlockchainB;
use crate::merkle_tree::Hash;

/// The `Relay` structure facilitates cross-chain proof submission.
///
/// This structure acts as a relay mechanism, allowing a proof generated in one blockchain
/// to be submitted to another blockchain for verification. It is a crucial part of 
/// cross-chain interoperability.
pub struct Relay;

impl Relay {
    /// Submits a cryptographic proof to another blockchain for verification.
    ///
    /// This function takes a Merkle proof and submits it to a different blockchain 
    /// (represented here as `BlockchainB`) for verification. The verification logic
    /// resides in the target blockchain.
    ///
    /// # Arguments
    ///
    /// * `proof` - A vector of `Hash` values representing the Merkle proof to be verified.
    /// * `blockchain_b` - A mutable reference to the target blockchain (`BlockchainB`) where the proof will be submitted for verification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::relay::Relay;
    /// use crate::blockchain::BlockchainB;
    /// use crate::merkle_tree::Hash;
    /// use std::collections::HashMap;
    ///
    /// let mut blockchain_b = BlockchainB {
    ///     known_roots: HashMap::new(),
    /// };
    ///
    /// let proof: Vec<Hash> = vec![vec![0u8; 32]];
    ///
    /// Relay::submit_proof(proof, &mut blockchain_b);
    /// ```
    pub fn submit_proof(proof: Vec<Hash>, blockchain_b: &mut BlockchainB) {
        // Submits the proof to BlockchainB for verification
        blockchain_b.verify_proof(proof);
    }
}
