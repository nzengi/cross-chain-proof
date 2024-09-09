use crate::blockchain::BlockchainB;
use crate::merkle_tree::Hash;

/// Relay structure for cross-chain proof submission
pub struct Relay;

impl Relay {
    /// Submits the proof to another blockchain for verification
    pub fn submit_proof(proof: Vec<Hash>, blockchain_b: &mut BlockchainB) {
        blockchain_b.verify_proof(proof);
    }
}
