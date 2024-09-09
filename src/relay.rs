use crate::blockchain::BlockchainB;
use crate::merkle_tree::Hash;

/// The `Relay` structure facilitates cross-chain proof submission.
///
/// This structure acts as a relay mechanism, allowing a proof generated in one blockchain
/// to be submitted to another blockchain for verification. It is a crucial part of 
/// cross-chain interoperability.
/// Relay structure for cross-chain proof submission
pub struct Relay;

impl Relay {
    /// Submits the proof to another blockchain for verification
    ///
    /// # Example
    /// 
    /// ```rust
    /// use cross_chain_merkle_proof::relay::Relay;
    /// use cross_chain_merkle_proof::blockchain::BlockchainB;
    /// use cross_chain_merkle_proof::merkle_tree::Hash;
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
        blockchain_b.verify_proof(proof);
    }
}

