// aes.rs

// Import the RNS module to use RNS functions
use crate::rns::{rns_encode, rns_add};  // Removed unused rns_multiply import

/// Converts a byte slice (typically a 16-byte block) into a u64
fn block_to_u64(block: &[u8]) -> u64 {
    let mut result = 0u64;
    for (i, &byte) in block.iter().take(8).enumerate() {
        result |= (byte as u64) << (i * 8);
    }
    result
}


/// AES Block Encryption using RNS
/// Note: `_key` is kept in the function signature for now, but it’s not being used
pub fn encrypt_block(block: &[u8], _key: &[u8], moduli: &[u64]) -> Vec<u64> {
    let encoded_block = rns_encode(block_to_u64(block), moduli);
    let encrypted_block = rns_add(&encoded_block, &encoded_block);  // Placeholder
    encrypted_block
}
