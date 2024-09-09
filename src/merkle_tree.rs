extern crate sha2;
use sha2::{Digest, Sha256};

/// Type alias for Hashes
pub type Hash = Vec<u8>;

/// Structure representing a node in the Merkle Tree
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: Hash,
    pub left: Option<Box<MerkleNode>>,  // Left child node
    pub right: Option<Box<MerkleNode>>, // Right child node
}

/// Structure representing the Merkle Tree
#[derive(Debug)]
pub struct MerkleTree {
    pub root: Option<Box<MerkleNode>>,  // Root node of the tree
    pub leaves: Vec<MerkleNode>,        // List of leaf nodes
}

impl MerkleTree {
    /// Creates a new Merkle Tree from the given leaves data
    pub fn new(leaves_data: Vec<Hash>) -> Self {
        let mut leaves: Vec<MerkleNode> = leaves_data.into_iter().map(|data| {
            MerkleNode {
                hash: compute_hash(&data),
                left: None,
                right: None,
            }
        }).collect();

        let root = MerkleTree::build_tree(&mut leaves);
        MerkleTree { root, leaves }
    }

    /// Recursively builds the Merkle Tree by hashing pairs of nodes
    fn build_tree(nodes: &mut Vec<MerkleNode>) -> Option<Box<MerkleNode>> {
        if nodes.len() == 1 {
            return Some(Box::new(nodes.remove(0)));
        }

        // If odd number of nodes, duplicate the last node
        if nodes.len() % 2 != 0 {
            let last = nodes.last().cloned().unwrap();
            nodes.push(last);
        }

        let mut parents = Vec::new();
        while !nodes.is_empty() {
            let left = nodes.remove(0);
            let right = nodes.remove(0);

            let combined_hash = compute_hash(&[left.hash.clone(), right.hash.clone()].concat());
            parents.push(MerkleNode {
                hash: combined_hash,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            });
        }

        MerkleTree::build_tree(&mut parents)
    }

    /// Returns the root hash of the Merkle Tree
    pub fn root_hash(&self) -> Option<Hash> {
        self.root.as_ref().map(|node| node.hash.clone())
    }

    /// Generates a proof for the given leaf node hash
    pub fn generate_proof(&self, target_hash: &Hash) -> Vec<Hash> {
        let mut proof = Vec::new();
        MerkleTree::find_proof(&self.root, target_hash, &mut proof);
        proof
    }

    /// Recursively finds the proof for the target hash
    fn find_proof(node: &Option<Box<MerkleNode>>, target_hash: &Hash, proof: &mut Vec<Hash>) -> bool {
        if let Some(ref n) = node {
            if &n.hash == target_hash {
                return true;
            }

            if MerkleTree::find_proof(&n.left, target_hash, proof) {
                if let Some(ref right) = n.right {
                    proof.push(right.hash.clone());
                }
                return true;
            }

            if MerkleTree::find_proof(&n.right, target_hash, proof) {
                if let Some(ref left) = n.left {
                    proof.push(left.hash.clone());
                }
                return true;
            }
        }
        false
    }
}

/// Helper function to compute a SHA-256 hash
pub fn compute_hash(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_proof() {
        // Test verisi olarak 3 yaprak düğümü ekliyoruz
        let leaves = vec![vec![0u8; 32], vec![1u8; 32], vec![2u8; 32]];
        let merkle_tree = MerkleTree::new(leaves.clone()); // leaves'leri klonluyoruz

        // İlk yaprak düğümünü alıyoruz ve hash'ini üretiyoruz
        let target_leaf = compute_hash(&leaves[0]); // İlk yaprağın hash'i hedef

        // İlk yaprak düğümü için kanıt oluşturmayı test ediyoruz
        let proof = merkle_tree.generate_proof(&target_leaf);
        
        // Kanıtın boş olmadığından emin oluyoruz
        assert!(!proof.is_empty(), "Proof should not be empty");

        // Kanıtın doğru hash'i içerdiğini kontrol edebilirsiniz
        // Örneğin: assert_eq!(proof[0], expected_hash);
    }
}
