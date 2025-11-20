// Program to generate complete kat.rs with golden values
// Run with: cargo run --features "ml-kem,ml-dsa,fips_140_3" --bin update_kat_rs

use pqc_nostd::{
    kyber_generate_key_pair_internal, kyber_encapsulate_internal,
    dilithium_generate_key_pair_internal, dilithium_sign_internal,
    FIPS_CONTEXT,
    ML_KEM_1024_PK_BYTES, ML_KEM_1024_SK_BYTES, ML_KEM_1024_CT_BYTES, ML_KEM_1024_SS_BYTES,
    ML_DSA_65_PK_BYTES, ML_DSA_65_SK_BYTES, ML_DSA_65_SIG_BYTES,
};

fn main() {
    // Generate all values
    let ml_kem_seed = [0xAAu8; 64];
    let ml_kem_kp = kyber_generate_key_pair_internal(ml_kem_seed);
    let ml_kem_randomness = [0xBBu8; 32];
    let (ct, ss) = kyber_encapsulate_internal(&ml_kem_kp.public_key(), ml_kem_randomness);

    let ml_dsa_seed = [0xCCu8; 32];
    let ml_dsa_kp = dilithium_generate_key_pair_internal(ml_dsa_seed);
    let msg = b"FIPS 140-3 KAT";
    let ml_dsa_randomness = [0xDDu8; 32];
    let sig = dilithium_sign_internal(&ml_dsa_kp.signing_key, msg, FIPS_CONTEXT, ml_dsa_randomness)
        .expect("Signing failed");

    // Print the complete file
    print_file_header();
    print_ml_kem_section(ml_kem_kp.public_key().as_ref(), ml_kem_kp.private_key().as_ref(), ct.as_ref(), &ss);
    print_ml_dsa_section(ml_dsa_kp.verification_key.as_ref(), ml_dsa_kp.signing_key.as_ref(), sig.as_ref());
    print_file_footer();
}

fn print_file_header() {
    println!(r#"// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/kat.rs
//! FIPS 140-3 Known Answer Tests (KATs).
//!
//! These tests run at power-up (part of POST) and verify the correctness
//! of the cryptographic algorithms using fixed inputs and known outputs.

use crate::error::{{PqcError, Result}};

#[cfg(feature = "ml-kem")]
use crate::{{
    kyber_generate_key_pair_internal, kyber_encapsulate_internal, kyber_decapsulate_internal,
    KyberPublicKey, KyberPrivateKey, KyberCiphertext, KyberSharedSecret,
    ML_KEM_1024_PK_BYTES, ML_KEM_1024_SK_BYTES, ML_KEM_1024_CT_BYTES, ML_KEM_1024_SS_BYTES,
}};

#[cfg(feature = "ml-dsa")]
use crate::{{
    dilithium_generate_key_pair_internal, dilithium_sign_internal, dilithium_verify_internal,
    DilithiumVerifyingKey, DilithiumSigningKey, DilithiumSignature, FIPS_CONTEXT,
    ML_DSA_65_PK_BYTES, ML_DSA_65_SK_BYTES, ML_DSA_65_SIG_BYTES,
}};

/// Runs all Known Answer Tests.
pub fn run_kats() -> Result<()> {{
    #[cfg(feature = "ml-kem")]
    ml_kem_kat()?;

    #[cfg(feature = "ml-dsa")]
    ml_dsa_kat()?;

    Ok(())
}}

#[cfg(feature = "ml-kem")]
fn ml_kem_kat() -> Result<()> {{
    // 1. KeyGen KAT
    let seed = [0xAAu8; 64]; // Fixed seed
    let kp = kyber_generate_key_pair_internal(seed);

    // Expected Public Key"#);
}

fn print_ml_kem_section(pk: &[u8], sk: &[u8], ct: &[u8], ss: &[u8]) {
    println!("    let expected_pk: [u8; ML_KEM_1024_PK_BYTES] = [");
    print_hex_array(pk);
    println!("    ];");
    println!();
    println!("    // Expected Private Key");
    println!("    let expected_sk: [u8; ML_KEM_1024_SK_BYTES] = [");
    print_hex_array(sk);
    println!("    ];");
    println!();
    println!(r#"    if kp.public_key().as_ref() != expected_pk {{
        return Err(PqcError::KatFailure);
    }}
    if kp.private_key().as_ref() != expected_sk {{
        return Err(PqcError::KatFailure);
    }}

    // 2. Encapsulate KAT
    let randomness = [0xBBu8; 32]; // Fixed randomness
    let (ct, ss) = kyber_encapsulate_internal(&kp.public_key(), randomness);

    // Expected Ciphertext"#);
    println!("    let expected_ct: [u8; ML_KEM_1024_CT_BYTES] = [");
    print_hex_array(ct);
    println!("    ];");
    println!();
    println!("    // Expected Shared Secret");
    println!("    let expected_ss: [u8; ML_KEM_1024_SS_BYTES] = [");
    print_hex_array(ss);
    println!("    ];");
    println!();
    println!(r#"    if ct.as_ref() != expected_ct {{
        return Err(PqcError::KatFailure);
    }}
    if ss != expected_ss {{
        return Err(PqcError::KatFailure);
    }}

    // 3. Decapsulate KAT
    let ss_decap = kyber_decapsulate_internal(&kp.private_key(), &ct);
    if ss_decap != expected_ss {{
        return Err(PqcError::KatFailure);
    }}

    Ok(())
}}

#[cfg(feature = "ml-dsa")]
fn ml_dsa_kat() -> Result<()> {{
    // 1. KeyGen KAT
    let seed = [0xCCu8; 32]; // Fixed seed
    let kp = dilithium_generate_key_pair_internal(seed);

    // Expected Verifying Key"#);
}

fn print_ml_dsa_section(vk: &[u8], sk: &[u8], sig: &[u8]) {
    println!("    let expected_vk: [u8; ML_DSA_65_PK_BYTES] = [");
    print_hex_array(vk);
    println!("    ];");
    println!();
    println!("    // Expected Signing Key");
    println!("    let expected_sk: [u8; ML_DSA_65_SK_BYTES] = [");
    print_hex_array(sk);
    println!("    ];");
    println!();
    println!(r#"    if kp.verification_key.as_ref() != expected_vk {{
        return Err(PqcError::KatFailure);
    }}
    if kp.signing_key.as_ref() != expected_sk {{
        return Err(PqcError::KatFailure);
    }}

    // 2. Sign KAT
    let msg = b"FIPS 140-3 KAT";
    let randomness = [0xDDu8; 32]; // Fixed randomness
    let sig = dilithium_sign_internal(&kp.signing_key, msg, FIPS_CONTEXT, randomness)
        .map_err(|_| PqcError::KatFailure)?;

    // Expected Signature"#);
    println!("    let expected_sig: [u8; ML_DSA_65_SIG_BYTES] = [");
    print_hex_array(sig);
    println!("    ];");
    println!();
    println!(r#"    if sig.as_ref() != expected_sig {{
        return Err(PqcError::KatFailure);
    }}

    // 3. Verify KAT
    dilithium_verify_internal(&kp.verification_key, msg, FIPS_CONTEXT, &sig)
        .map_err(|_| PqcError::KatFailure)?;

    Ok(())
}}"#);
}

fn print_file_footer() {
    // Nothing needed - file ends with the ml_dsa_kat function
}

fn print_hex_array(data: &[u8]) {
    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 {
            print!("        ");
        }
        print!("0x{:02x}, ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    if data.len() % 16 != 0 {
        println!();
    }
}
