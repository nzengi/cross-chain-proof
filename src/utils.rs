use sha2::{Digest, Sha256};

/// Computes the SHA-256 hash of the input data.
///
/// This helper function takes a byte slice (`&[u8]`) as input, computes the 
/// SHA-256 hash using the `sha2` crate, and returns the hash as a vector of bytes.
///
/// # Arguments
///
/// * `data` - A byte slice representing the input data to be hashed.
///
/// # Returns
///
/// * `Vec<u8>` - A vector of bytes representing the SHA-256 hash of the input data.
///
/// # Example
///
/// ```rust
/// let data = b"hello world";
/// let hash = compute_sha256(data);
/// println!("SHA-256 hash: {:?}", hash);
/// ```
///
/// This will print the SHA-256 hash of "hello world" in hexadecimal format.
pub fn compute_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();  // Initialize a new SHA-256 hasher
    hasher.update(data);             // Feed the input data into the hasher
    hasher.finalize().to_vec()       // Retrieve the final hash result as a Vec<u8>
}
