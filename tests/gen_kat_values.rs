use pqc_nostd::auth::{login, Role};
use pqc_nostd::*;

#[test]
fn generate_kat_values() {
    println!("--- BEGIN KAT VALUES ---");

    // Authenticate as User to allow operations
    login(Role::User, b"user123").unwrap();

    // ML-KEM-1024
    #[cfg(feature = "ml-kem")]
    {
        let seed = [0xAAu8; 64];
        let kp = kyber_generate_key_pair(seed).unwrap();
        println!("ML-KEM Seed: {:02x?}", seed);

        println!("// ML-KEM Public Key");
        print_byte_array(kp.public_key().as_ref());
        println!("// ML-KEM Private Key");
        print_byte_array(kp.private_key().as_ref());

        let randomness = [0xBBu8; 32];
        let (ct, ss) = encapsulate(kp.public_key(), randomness).unwrap();
        println!("// ML-KEM Ciphertext");
        print_byte_array(ct.as_ref());
        println!("// ML-KEM Shared Secret");
        print_byte_array(&ss);
    }

    // ML-DSA-65
    #[cfg(feature = "ml-dsa")]
    {
        let seed = [0xCCu8; 32];
        let kp = dilithium_generate_key_pair(seed).unwrap();
        println!("// ML-DSA Verifying Key");
        print_byte_array(kp.verification_key.as_ref());
        println!("// ML-DSA Signing Key");
        print_byte_array(kp.signing_key.as_ref());

        let msg = b"FIPS 140-3 KAT";
        let ctx = FIPS_CONTEXT; // or empty
        let randomness = [0xDDu8; 32];
        let sig = dilithium_sign(&kp.signing_key, msg, ctx, randomness).unwrap();

        println!("// ML-DSA Signature");
        print_byte_array(sig.as_ref());
    }

    println!("--- END KAT VALUES ---");
}

fn print_byte_array(bytes: &[u8]) {
    print!("[");
    for (i, b) in bytes.iter().enumerate() {
        if i % 16 == 0 {
            print!("\n    ");
        }
        print!("0x{:02x}, ", b);
    }
    println!("\n]");
}
