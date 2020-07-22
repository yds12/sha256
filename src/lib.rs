//! This library implements the SHA-256 cryptographic hash function.

extern crate bit_vec;
use bit_vec::BitVec;
use std::convert::TryInto;

/// Returns a SHA-256 hash of the string `hash_me`.
///
/// ```
/// let s = "secret text";
/// let encrypted = sha256::from_string(s);
///
/// let s2 = "pão";
/// let encrypted2 = sha256::from_string(s2);
///
/// assert_eq!(encrypted.len(), 32);
/// assert_eq!(encrypted2.len(), 32);
/// ```
pub fn from_string(hash_me: &str) -> String {
    // Original message as a bit vector
    let mut bits = BitVec::from_bytes(hash_me.as_bytes());
    let bits_len: u64 = bits.len().try_into().unwrap();

    // SHA-256 constants
    let h0: u32 = 0x6a09e667;
    let h1: u32 = 0xbb67ae85;
    let h2: u32 = 0x3c6ef372;
    let h3: u32 = 0xa54ff53a;
    let h4: u32 = 0x510e527f;
    let h5: u32 = 0x9b05688c;
    let h6: u32 = 0x1f83d9ab;
    let h7: u32 = 0x5be0cd19;

    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 ];

    // Append 1 to the end of the message
    bits.push(true);

    // Original msg of len L
    // append K 0 bits, where K is the min number s.t. L+1+K+64 is a
    // multiple of 512
    let n_zeroes = (512 - ((bits_len + 65) % 512)) % 512;
    let mut zeroes = BitVec::from_elem(n_zeroes.try_into().unwrap(), false);
    bits.append(&mut zeroes);

    // append L as a 64-bit big-endian integer (total size now is multiple
    // of 512 bits)
    let mut size_bits = BitVec::from_bytes(&bits_len.to_be_bytes());
    bits.append(&mut size_bits);
    println!("{:?}", bits);

    String::new()
}
