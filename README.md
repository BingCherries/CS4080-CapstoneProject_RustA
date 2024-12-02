# Class: CS4080-CapstoneProject_RustA
# Team: Rust A

Reference: RustCrypto/block-ciphers (https://github.com/RustCrypto/block-ciphers)

# Topic: RustCrypto AES implementation to incorporate Residue Number System (RNS)
- AES is a symmetric encryption algorithm that involves key expansion, substitution, permutation, and multiple rounds of encryption.
- The current AES implementation in RustCrypto likely uses byte-level operations for the encryption and decryption process, using well-established block cipher modes (e.g., ECB, CBC).
- This project explores the integration of Residue Number System (RNS) with AES to potentially enhance the performance and efficiency of cryptographic operations. RNS represents numbers in modular form based on a set of moduli, allowing for parallel computation. By leveraging RNS, we aim to accelerate key operations in AES, such as modular arithmetic, which could lead to reduced computation times and increased throughput for encryption and decryption.


# Key Objectives
We'll focus on modifying specific parts where modular arithmetic or key operations occur.
1. Define RNS Functions
   - Implement functions for RNS encoding, addition, and multiplication. These operations will be used to replace some of the traditional modular arithmetic operations in AES.
2. Integrating RNS with AES
   - Modify the AES encryption and decryption process to incorporate the RNS functions, optimizing key operations and reducing the overall computational overhead.
3. Testing RNS methods
   - Develop unit tests to verify the correctness and performance improvements brought by the RNS integration in AES encryption.

# Modifications from the Reference Implementation
In this project, we modified the original RustCrypto AES implementation to incorporate Residue Number System (RNS). 
1. New RNS Functions Added
   - rns_encode: This function breaks a value into residues based on a list of moduli. It performs modular arithmetic for each modulus.
   - rns_add: Adds two RNS-encoded numbers element-wise, using modular addition.
   - rns_multiply: Multiplies two RNS-encoded numbers element-wise, performing modular multiplication for each pair.
2. AES Key Operations Modified:
   - The original AES implementation relies on byte-level modular arithmetic. We modified the key expansion and other modular operations to use RNS functions, enabling parallel computations for faster processing.
3. Integration of RNS into AES Encryption Process:
   - Instead of performing traditional byte-level modular arithmetic, the AES implementation now uses the RNS-encoded values to carry out the cryptographic operations. This enhances the performance of encryption and decryption tasks.


# Why Use RNS in AES?
The Residue Number System (RNS) is a method of representing numbers as a set of residues, each corresponding to a modulus from a given list. This approach has several advantages in cryptography:
- Parallel Computation: RNS enables modular arithmetic to be done independently across multiple moduli, allowing for parallel processing and faster execution.
Efficient Cryptographic Operations: The AES algorithm, which heavily involves modular arithmetic, can benefit from RNS by reducing the cycles required for operations like encryption and decryption.
