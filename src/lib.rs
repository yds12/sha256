//! This library implements the SHA-256 cryptographic hash function.

/// Returns a SHA-256 hash of the string `hash_me`.
///
/// ```
/// let s = "secret text";
/// let encrypted = sha256::from_string(s);
/// assert_eq!(encrypted.len(), 32);
/// ```
pub fn from_string(hash_me: &str) -> String {
    String::new();
}
