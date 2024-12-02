// main.rs

mod aes;
mod rns;

fn main() {
    let key = b"mysecretkey";
    let block = b"plaintext";
    let moduli = vec![255, 256]; // Example moduli for RNS

    // Encrypt the block using AES encryption (which internally uses RNS operations)
    let encrypted_block = aes::encrypt_block(block, key, &moduli);
    
    // Print the encrypted result
    println!("{:?}", encrypted_block);
}
