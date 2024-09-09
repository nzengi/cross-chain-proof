use sha2::{Digest, Sha256};

/// Helper function to compute a SHA-256 hash of input data
pub fn compute_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
