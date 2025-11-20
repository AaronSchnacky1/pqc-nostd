// Temporary program to generate KAT values and print them in a format ready for kat.rs
// Run with: cargo run --features "ml-kem,ml-dsa,fips_140_3" --bin gen_kat_simple

use pqc_nostd::{
    kyber_generate_key_pair_internal, kyber_encapsulate_internal,
    dilithium_generate_key_pair_internal, dilithium_sign_internal,
    FIPS_CONTEXT,
};

fn main() {
    // Generate ML-KEM values
    let ml_kem_seed = [0xAAu8; 64];
    let ml_kem_kp = kyber_generate_key_pair_internal(ml_kem_seed);
    let ml_kem_randomness = [0xBBu8; 32];
    let (ct, ss) = kyber_encapsulate_internal(&ml_kem_kp.public_key(), ml_kem_randomness);

    // Generate ML-DSA values
    let ml_dsa_seed = [0xCCu8; 32];
    let ml_dsa_kp = dilithium_generate_key_pair_internal(ml_dsa_seed);
    let msg = b"FIPS 140-3 KAT";
    let ml_dsa_randomness = [0xDDu8; 32];
    let sig = dilithium_sign_internal(&ml_dsa_kp.signing_key, msg, FIPS_CONTEXT, ml_dsa_randomness)
        .expect("Signing failed");

    // Print ML-KEM Ciphertext
    println!("ML-KEM Ciphertext:");
    print_hex_array(ct.as_ref());
    println!();

    // Print ML-KEM Shared Secret
    println!("ML-KEM Shared Secret:");
    print_hex_array(&ss);
    println!();

    // Print ML-DSA Verification Key
    println!("ML-DSA Verification Key:");
    print_hex_array(ml_dsa_kp.verification_key.as_ref());
    println!();

    // Print ML-DSA Signing Key
    println!("ML-DSA Signing Key:");
    print_hex_array(ml_dsa_kp.signing_key.as_ref());
    println!();

    // Print ML-DSA Signature
    println!("ML-DSA Signature:");
    print_hex_array(sig.as_ref());
    println!();
}

fn print_hex_array(data: &[u8]) {
    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 {
            if i > 0 {
                println!();
            }
            print!("    ");
        }
        print!("0x{:02x}", byte);
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!();
}
