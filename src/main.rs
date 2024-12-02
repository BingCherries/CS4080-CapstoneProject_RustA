use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use std::str;
use rsa::PublicKey; // Add this import to bring the `encrypt` method into scope

fn generate_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    // Generate a new RSA key pair
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    
    (private_key, public_key)
}

fn encrypt_data(public_key: &RsaPublicKey, data: &str) -> Vec<u8> {
    let mut rng = OsRng;
    
    // Encrypt the data using the public key with PKCS1v15 padding scheme
    public_key.encrypt(
        &mut rng,
        PaddingScheme::new_pkcs1v15_encrypt(), // Updated method for PKCS1v15 padding
        data.as_bytes(),
    ).expect("failed to encrypt")
}

fn decrypt_data(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> String {
    // Decrypt the data using the private key
    let decrypted_data = private_key.decrypt(
        PaddingScheme::new_pkcs1v15_encrypt(), // Updated method for PKCS1v15 padding
        encrypted_data,
    ).expect("failed to decrypt");

    // Convert the decrypted bytes back to a string
    String::from_utf8(decrypted_data).expect("failed to convert decrypted data to string")
}

fn main() {
    // Generate RSA keys
    let (private_key, public_key) = generate_rsa_keys();

    // Sample data to encrypt
    let data = "Hello, RSA encryption!";
    // let data = "This is Test";
    
    // Print the initial message (data)
    println!("\nInitial data: {}\n", data);

    // Encrypt the data using the public key
    let encrypted_data = encrypt_data(&public_key, data);
    println!("Encrypted data: {:?}\n", encrypted_data);
    
    // Decrypt the data using the private key
    let decrypted_data = decrypt_data(&private_key, &encrypted_data);
    println!("Decrypted data: {}", decrypted_data);
}
