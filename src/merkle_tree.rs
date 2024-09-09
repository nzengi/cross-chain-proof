extern crate sha2;
use sha2::{Digest, Sha256};

/// Type alias for Hashes, represented as a vector of bytes (u8)
pub type Hash = Vec<u8>;

/// Structure representing a node in the Merkle Tree
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: Hash,                           // Hash of the node (leaf or internal node)
    pub left: Option<Box<MerkleNode>>,        // Left child node (None if leaf)
    pub right: Option<Box<MerkleNode>>,       // Right child node (None if leaf)
}

/// Structure representing the Merkle Tree
#[derive(Debug)]
pub struct MerkleTree {
    pub root: Option<Box<MerkleNode>>,        // Root node of the tree
    pub leaves: Vec<MerkleNode>,              // List of leaf nodes (original data)
}

impl MerkleTree {
    /// Creates a new Merkle Tree from the given leaf data
    ///
    /// Takes a vector of hashed leaves (data) and builds a Merkle Tree,
    /// returning the root node of the tree.
    ///
    /// # Arguments
    ///
    /// * `leaves_data` - A vector of hashes representing the leaf data of the Merkle Tree.
    ///
    /// # Returns
    ///
    /// * `MerkleTree` - A Merkle Tree with the root node and the list of leaves.
    pub fn new(leaves_data: Vec<Hash>) -> Self {
        // Convert the leaf data into Merkle nodes
        let mut leaves: Vec<MerkleNode> = leaves_data.into_iter().map(|data| {
            MerkleNode {
                hash: compute_hash(&data),  // Compute hash of each leaf node
                left: None,
                right: None,
            }
        }).collect();

        // Build the Merkle tree from the leaves
        let root = MerkleTree::build_tree(&mut leaves);
        MerkleTree { root, leaves }
    }

    /// Recursively builds the Merkle Tree by hashing pairs of nodes
    ///
    /// Starting with the leaf nodes, this function recursively hashes pairs
    /// of nodes to create parent nodes, building up to the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A mutable vector of `MerkleNode` representing the current level of the tree.
    ///
    /// # Returns
    ///
    /// * `Option<Box<MerkleNode>>` - The root node of the Merkle Tree.
    fn build_tree(nodes: &mut Vec<MerkleNode>) -> Option<Box<MerkleNode>> {
        if nodes.len() == 1 {
            return Some(Box::new(nodes.remove(0)));  // Return the single remaining node as the root
        }

        // If odd number of nodes, duplicate the last node to make the count even
        if nodes.len() % 2 != 0 {
            let last = nodes.last().cloned().unwrap();
            nodes.push(last);
        }

        let mut parents = Vec::new();  // Parent nodes for the next level
        while !nodes.is_empty() {
            let left = nodes.remove(0);  // Remove left node
            let right = nodes.remove(0); // Remove right node

            // Hash the concatenation of left and right node hashes
            let combined_hash = compute_hash(&[left.hash.clone(), right.hash.clone()].concat());
            parents.push(MerkleNode {
                hash: combined_hash,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            });
        }

        // Recursively build the next level
        MerkleTree::build_tree(&mut parents)
    }

    /// Returns the root hash of the Merkle Tree
    ///
    /// If the Merkle Tree is valid, this function returns the hash of the root node.
    ///
    /// # Returns
    ///
    /// * `Option<Hash>` - The hash of the root node or None if the tree is empty.
    pub fn root_hash(&self) -> Option<Hash> {
        self.root.as_ref().map(|node| node.hash.clone())
    }

    /// Generates a proof for the given leaf node hash
    ///
    /// This function generates a cryptographic proof (path of hashes) for a given leaf node,
    /// allowing a verifier to confirm the inclusion of this node in the Merkle Tree.
    ///
    /// # Arguments
    ///
    /// * `target_hash` - The hash of the leaf node for which to generate a proof.
    ///
    /// # Returns
    ///
    /// * `Vec<Hash>` - A vector of hashes representing the proof.
    pub fn generate_proof(&self, target_hash: &Hash) -> Vec<Hash> {
        let mut proof = Vec::new();
        MerkleTree::find_proof(&self.root, target_hash, &mut proof);
        proof
    }

    /// Recursively finds the proof for the target hash
    ///
    /// This function traverses the Merkle Tree to find the path of hashes
    /// (proof) for a given target leaf node.
    ///
    /// # Arguments
    ///
    /// * `node` - The current node being checked.
    /// * `target_hash` - The hash of the leaf node to find.
    /// * `proof` - A mutable vector to store the proof hashes.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the target hash was found, otherwise False.
    fn find_proof(node: &Option<Box<MerkleNode>>, target_hash: &Hash, proof: &mut Vec<Hash>) -> bool {
        if let Some(ref n) = node {
            // If the current node's hash matches the target, proof is complete
            if &n.hash == target_hash {
                return true;
            }

            // Recursively search the left subtree
            if MerkleTree::find_proof(&n.left, target_hash, proof) {
                if let Some(ref right) = n.right {
                    proof.push(right.hash.clone());  // Add right sibling hash to proof
                }
                return true;
            }

            // Recursively search the right subtree
            if MerkleTree::find_proof(&n.right, target_hash, proof) {
                if let Some(ref left) = n.left {
                    proof.push(left.hash.clone());  // Add left sibling hash to proof
                }
                return true;
            }
        }
        false
    }
}

/// Helper function to compute a SHA-256 hash
///
/// This function computes the SHA-256 hash of the given data.
/// It is used to generate the hashes for each node in the Merkle Tree.
///
/// # Arguments
///
/// * `data` - A slice of bytes to hash.
///
/// # Returns
///
/// * `Hash` - The resulting hash as a vector of bytes.
pub fn compute_hash(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the generation of a Merkle proof for a specific leaf node
    #[test]
    fn test_merkle_tree_proof() {
        // Create test leaf data with 3 nodes
        let leaves = vec![vec![0u8; 32], vec![1u8; 32], vec![2u8; 32]];
        let merkle_tree = MerkleTree::new(leaves.clone());  // Clone leaves

        // Compute the hash for the first leaf node (target)
        let target_leaf = compute_hash(&leaves[0]);

        // Generate a proof for the first leaf node
        let proof = merkle_tree.generate_proof(&target_leaf);
        
        // Ensure the proof is not empty
        assert!(!proof.is_empty(), "Proof should not be empty");

        // You can also check the exact values in the proof if needed
        // assert_eq!(proof[0], expected_hash);
    }
}
