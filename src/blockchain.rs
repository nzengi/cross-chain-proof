use std::collections::HashMap;

pub struct Blockchain {
    pub known_roots: HashMap<u64, Vec<u8>>,  // Stores Merkle root hashes
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            known_roots: HashMap::new(),
        }
    }

    pub fn add_root(&mut self, id: u64, root_hash: Vec<u8>) {
        self.known_roots.insert(id, root_hash);
    }

    pub fn verify_proof(&self, proof: Vec<Vec<u8>>, leaf: Vec<u8>) -> bool {
        proof.contains(&leaf)  // Simplified proof verification logic
    }
}
