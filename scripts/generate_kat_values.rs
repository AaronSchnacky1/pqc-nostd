// scripts/generate_kat_values.rs
// Helper program to generate KAT golden values in chunks for easy copying

use pqc_nostd::{
    kyber_generate_key_pair_internal, kyber_encapsulate_internal,
    dilithium_generate_key_pair_internal, dilithium_sign_internal,
    FIPS_CONTEXT,
};

fn main() {
    println!("{}", "=".repeat(80));
    println!("KAT Golden Value Generator");
    println!("{}", "=".repeat(80));
    println!();

    // ML-KEM-1024 KATs
    println!("// ========================================");
    println!("// ML-KEM-1024 KAT VALUES");
    println!("// ========================================");
    println!();

    // 1. Generate ML-KEM key pair
    let ml_kem_seed = [0xAAu8; 64];
    let ml_kem_kp = kyber_generate_key_pair_internal(ml_kem_seed);
    
    println!("// ML-KEM-1024 Public Key already exists in kat.rs (lines 47-146)");
    println!("// ML-KEM-1024 Private Key already exists in kat.rs (lines 149-357)");
    println!();

    // 2. Generate ML-KEM encapsulation
    let ml_kem_randomness = [0xBBu8; 32];
    let (ct, ss) = kyber_encapsulate_internal(ml_kem_kp.public_key(), ml_kem_randomness);

    println!("// ML-KEM-1024 Ciphertext (1568 bytes) - REPLACE lines 371-374");
    println!("let expected_ct: [u8; ML_KEM_1024_CT_BYTES] = [");
    print_bytes_in_chunks(ct.as_ref(), 4, "    ");
    println!("];");
    println!();

    println!("// ML-KEM-1024 Shared Secret (32 bytes) - REPLACE lines 377-380");
    println!("let expected_ss: [u8; ML_KEM_1024_SS_BYTES] = [");
    print_bytes_in_chunks(&ss, 1, "    ");
    println!("];");
    println!();

    // ML-DSA-65 KATs
    println!("// ========================================");
    println!("// ML-DSA-65 KAT VALUES");
    println!("// ========================================");
    println!();

    // 1. Generate ML-DSA key pair
    let ml_dsa_seed = [0xCCu8; 32];
    let ml_dsa_kp = dilithium_generate_key_pair_internal(ml_dsa_seed);

    println!("// ML-DSA-65 Verification Key (1952 bytes) - REPLACE lines 405-408");
    println!("let expected_vk: [u8; ML_DSA_65_PK_BYTES] = [");
    print_bytes_in_chunks(ml_dsa_kp.verification_key.as_ref(), 4, "    ");
    println!("];");
    println!();

    println!("// ML-DSA-65 Signing Key (4032 bytes) - REPLACE lines 411-414");
    println!("let expected_sk: [u8; ML_DSA_65_SK_BYTES] = [");
    print_bytes_in_chunks(ml_dsa_kp.signing_key.as_ref(), 8, "    ");
    println!("];");
    println!();

    // 2. Generate ML-DSA signature
    let msg = b"FIPS 140-3 KAT";
    let ml_dsa_randomness = [0xDDu8; 32];
    let sig = dilithium_sign_internal(&ml_dsa_kp.signing_key, msg, FIPS_CONTEXT, ml_dsa_randomness)
        .expect("Signing failed");

    println!("// ML-DSA-65 Signature (3309 bytes) - REPLACE lines 430-433");
    println!("let expected_sig: [u8; ML_DSA_65_SIG_BYTES] = [");
    print_bytes_in_chunks(sig.as_ref(), 6, "    ");
    println!("];");
    println!();

    println!("{}", "=".repeat(80));
    println!("KAT Golden Values Generated Successfully!");
    println!("{}", "=".repeat(80));
    println!();
    println!("INSTRUCTIONS:");
    println!("1. Copy each section above");
    println!("2. Replace the corresponding placeholder in src/kat.rs");
    println!("3. Verify line numbers match (they're indicated in comments)");
    println!("4. Run: cargo test --features \"ml-kem,ml-dsa,fips_140_3\"");
    println!();
}

/// Prints bytes in hex format, split into the specified number of chunks
fn print_bytes_in_chunks(data: &[u8], num_chunks: usize, indent: &str) {
    let chunk_size = data.len().div_ceil(num_chunks); // Round up
    
    for (chunk_idx, chunk) in data.chunks(chunk_size).enumerate() {
        print!("{}", indent);
        for (i, byte) in chunk.iter().enumerate() {
            print!("0x{:02x}", byte);
            if i < chunk.len() - 1 || chunk_idx < num_chunks - 1 {
                print!(", ");
            }
            // Line break every 16 bytes for readability
            if (i + 1) % 16 == 0 && i < chunk.len() - 1 {
                print!("\n{}", indent);
            }
        }
        println!();
    }
}
