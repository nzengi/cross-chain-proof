use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub leaves: Vec<Vec<u8>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Vec<u8>>) -> Self {
        MerkleTree { leaves }
    }

    pub fn root_hash(&self) -> Option<Vec<u8>> {
        if self.leaves.is_empty() {
            return None;
        }

        let mut hasher = Sha256::new();
        for leaf in &self.leaves {
            hasher.update(leaf);
        }

        Some(hasher.finalize().to_vec())
    }

    pub fn generate_proof(&self, _leaf: &[u8]) -> Vec<Vec<u8>> {
        self.leaves.clone()  // Simplified for demonstration
    }
    
}
