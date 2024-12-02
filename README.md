# CS4080: Concepts of Programming Languages (Capstone Project)
# Team: Rust A
Members: Cody Apolinar, Kelvin Quizon, Thomas Christopher Tejedor, Yunseon Choi

# RSA Encryption Project
## Project Goal
- This project demonstrates RSA encryption and decryption using the ```rsa``` crate in Rust. The goal is to show how to encrypt data using a public key and decrypt it using a private key. The project uses the PKCS#1 v1.5 padding scheme for encryption and decryption.

## Main Functionalities
- RSA Key Generation: Generate RSA keys (public and private).
- Encryption: Encrypt data using the public key.
- Decryption: Decrypt the encrypted data using the private key.
- Display Results: Show the initial data, encrypted data (in byte form), and the decrypted message.

## How it Works
1. Key Generation: RSA keys are generated with a 2048-bit size.
2. Encryption: The input message is encrypted with the public key.
3. Decryption: The encrypted message is decrypted back to the original message using the private key.

## How to Run
- Terminal:
  ```
  cargo run
  ```

- Output Example:
  ```
  Initial data: Hello, RSA encryption!
  Encrypted data: [67, 44, 77, 33, 49, 65, 25, 6, 72, 123, ...]  
  Decrypted data: Hello, RSA encryption!
  ```

## Modifications from the Reference
- Encryption Scheme: Using PKCS#1 v1.5 padding scheme for both encryption and decryption.
- No external libraries: Only ```rsa``` and ```rand``` crates are used.
- Key size: RSA key generation uses a 2048-bit key size for strong encryption.
