// tests/integration_test.rs
// Minimal integration test required for FIPS 140-3 submission
// Proves: POST passes → module enters Operational → approved algorithms work

use pqc_nostd::{run_post_or_panic, is_operational, kyber_generate_key_pair, dilithium_generate_key_pair, FIPS_CONTEXT};
use pqc_nostd::auth::{login, Role};

#[test]
fn fips_module_becomes_operational_and_algorithms_work() {
    run_post_or_panic();                    // Executes CASTs + PCTs
    assert!(is_operational(), "Module must be in Operational state after POST");

    // Level 2: Must login before operations
    login(Role::User, b"user123").expect("Login failed");

    // ML-KEM-1024 basic round-trip (fixed seeds – deterministic)
    let kyber_kp = kyber_generate_key_pair([0x11u8; 64]).unwrap();
    let (ct, ss1) = pqc_nostd::encapsulate(&kyber_kp.public_key(), [0x22u8; 32]).unwrap();
    let ss2 = pqc_nostd::decapsulate(&kyber_kp.private_key(), &ct).unwrap();
    assert_eq!(ss1, ss2);

    // ML-DSA-65 sign/verify (fixed seeds – deterministic)
    let dil_kp = dilithium_generate_key_pair([0x33u8; 32]).unwrap();
    let msg = b"FIPS 140-3 approved mode test";
    let sig = pqc_nostd::dilithium_sign(&dil_kp.signing_key, msg, FIPS_CONTEXT, [0x44u8; 32]).unwrap();
    assert!(pqc_nostd::dilithium_verify(&dil_kp.verification_key, msg, FIPS_CONTEXT, &sig).is_ok());
}