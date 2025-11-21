// Simple test to generate and verify KAT values work
#[cfg(test)]
#[cfg(all(feature = "ml-kem", feature = "ml-dsa", feature = "fips_140_3"))]
mod generate_kat_test {
    use pqc_nostd::{
        dilithium_generate_key_pair_internal, dilithium_sign_internal, dilithium_verify_internal,
        kyber_decapsulate_internal, kyber_encapsulate_internal, kyber_generate_key_pair_internal,
        FIPS_CONTEXT,
    };

    #[test]
    fn generate_ml_kem_values() {
        let seed = [0xAAu8; 64];
        let kp = kyber_generate_key_pair_internal(seed);
        let randomness = [0xBBu8; 32];
        let (ct, ss) = kyber_encapsulate_internal(kp.public_key(), randomness);

        println!("\n=== ML-KEM Ciphertext ===");
        for chunk in ct.as_ref().chunks(16) {
            print!("        ");
            for byte in chunk.iter() {
                print!("0x{:02x}, ", byte);
            }
            println!();
        }

        println!("\n=== ML-KEM Shared Secret ===");
        print!("        ");
        for byte in ss.iter() {
            print!("0x{:02x}, ", byte);
        }
        println!();

        // Verify decapsulation works
        let ss_decap = kyber_decapsulate_internal(kp.private_key(), &ct);
        assert_eq!(ss, ss_decap);
    }

    #[test]
    fn generate_ml_dsa_values() {
        let seed = [0xCCu8; 32];
        let kp = dilithium_generate_key_pair_internal(seed);

        println!("\n=== ML-DSA Verification Key ===");
        for chunk in kp.verification_key.as_ref().chunks(16) {
            print!("        ");
            for byte in chunk.iter() {
                print!("0x{:02x}, ", byte);
            }
            println!();
        }

        println!("\n=== ML-DSA Signing Key ===");
        for chunk in kp.signing_key.as_ref().chunks(16) {
            print!("        ");
            for byte in chunk.iter() {
                print!("0x{:02x}, ", byte);
            }
            println!();
        }

        let msg = b"FIPS 140-3 KAT";
        let randomness = [0xDDu8; 32];
        let sig = dilithium_sign_internal(&kp.signing_key, msg, FIPS_CONTEXT, randomness)
            .expect("Signing failed");

        println!("\n=== ML-DSA Signature ===");
        for chunk in sig.as_ref().chunks(16) {
            print!("        ");
            for byte in chunk.iter() {
                print!("0x{:02x}, ", byte);
            }
            println!();
        }

        // Verify signature works
        dilithium_verify_internal(&kp.verification_key, msg, FIPS_CONTEXT, &sig)
            .expect("Verification failed");
    }
}
