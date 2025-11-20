// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/lib.rs – FINAL GOLDEN VERSION
//! # PQC-NOSTD
//!
//! Post-Quantum Cryptography library for `no_std` environments.
//! Implements ML-KEM-1024 (FIPS 203) and ML-DSA-65 (FIPS 204).
//!
//! ## Features
//! - `ml-kem`: Enables ML-KEM-1024.
//! - `ml-dsa`: Enables ML-DSA-65.
//! - `fips_140_3`: Enables FIPS 140-3 Approved mode (POST, CASTs, state machine).
//!
//! ## Approved-Mode Usage
//!
//! This example demonstrates the correct initialization and usage of the module in FIPS Approved mode.
//!
//! ```rust
//! use pqc_nostd::{run_post_or_panic, is_operational, FIPS_CONTEXT};
//! use pqc_nostd::auth::{login, Role};
//!
//! fn main() {
//!     // 1. Power-On Self-Tests (POST) MUST be run before any crypto operation.
//!     run_post_or_panic();
//!     
//!     // 2. Verify the module is in the Operational state.
//!     assert!(is_operational());
//!
//!     // 3. Login as User (Level 2 Requirement)
//!     // In a real app, you would prompt for credentials.
//!     login(Role::User, b"user123").expect("Login failed");
//!
//!     // 4. Use Approved Algorithms
//!     
//!     // ML-KEM-1024 (Key Encapsulation)
//!     let kyber_kp = pqc_nostd::kyber_generate_key_pair([0x01u8; 64]).unwrap();
//!     let (ct, ss_alice) = pqc_nostd::encapsulate(&kyber_kp.public_key(), [0x02u8; 32]).unwrap();
//!     let ss_bob = pqc_nostd::decapsulate(&kyber_kp.private_key(), &ct).unwrap();
//!     assert_eq!(ss_alice, ss_bob);
//!
//!     // ML-DSA-65 (Digital Signatures)
//!     let dil_kp = pqc_nostd::dilithium_generate_key_pair([0x03u8; 32]).unwrap();
//!     let msg = b"FIPS 140-3 approved mode test";
//!     // Note: FIPS_CONTEXT is required for FIPS 204 compliance
//!     let sig = pqc_nostd::dilithium_sign(&dil_kp.signing_key, msg, FIPS_CONTEXT, [0x04u8; 32]).unwrap();
//!     assert!(pqc_nostd::dilithium_verify(&dil_kp.verification_key, msg, FIPS_CONTEXT, &sig).is_ok());
//! }
//! ```
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Error types and Result alias.
pub mod error;
/// Conditional Algorithm Self-Tests (CASTs).
pub mod cast;
/// FIPS module state management.
pub mod state;
/// Pair-wise Consistency Tests (PCTs).
pub mod pct;
/// Pre-operational self-tests (POST).
pub mod preop;
/// Critical Security Parameter (CSP) management.
/// Critical Security Parameter (CSP) management.
pub mod csp;
/// Role-Based Authentication (Level 2).
pub mod auth;
/// Software Integrity Test (Level 1/2).
pub mod integrity;

/// ML-KEM-1024 (FIPS 203) implementation.
#[cfg(feature = "ml-kem")]
pub mod ml_kem;
/// ML-DSA-65 (FIPS 204) implementation.
#[cfg(feature = "ml-dsa")]
pub mod ml_dsa;

// ML-KEM re-exports
#[cfg(feature = "ml-kem")]
pub use ml_kem::{
    encapsulate as kyber_encapsulate_internal,
    decapsulate as kyber_decapsulate_internal,
    generate_key_pair as kyber_generate_key_pair_internal,
    KyberCiphertext,
    KyberKeypair,
    KyberPublicKey,
    KyberPrivateKey,
    KyberSharedSecret,
};

use auth::{check_authority, Role};

/// Generates a Kyber key pair (Authenticated).
#[cfg(feature = "ml-kem")]
pub fn kyber_generate_key_pair(seed: [u8; 64]) -> Result<KyberKeypair> {
    check_authority(Role::User)?;
    Ok(kyber_generate_key_pair_internal(seed))
}

/// Encapsulates a shared secret (Authenticated).
#[cfg(feature = "ml-kem")]
pub fn encapsulate(pk: &KyberPublicKey, randomness: [u8; 32]) -> Result<(KyberCiphertext, KyberSharedSecret)> {
    check_authority(Role::User)?;
    Ok(kyber_encapsulate_internal(pk, randomness))
}

/// Decapsulates a shared secret (Authenticated).
#[cfg(feature = "ml-kem")]
pub fn decapsulate(sk: &KyberPrivateKey, ct: &KyberCiphertext) -> Result<KyberSharedSecret> {
    check_authority(Role::User)?;
    Ok(kyber_decapsulate_internal(sk, ct))
}

// ML-DSA re-exports
#[cfg(feature = "ml-dsa")]
pub use ml_dsa::{
    generate_key_pair as dilithium_generate_key_pair_internal,
    sign as dilithium_sign_internal,
    verify as dilithium_verify_internal,
    DilithiumSigningKey,
    DilithiumVerifyingKey,
    DilithiumSignature,
    DilithiumKeypair,
    FIPS_CONTEXT,
};

/// Generates a Dilithium key pair (Authenticated).
#[cfg(feature = "ml-dsa")]
pub fn dilithium_generate_key_pair(seed: [u8; 32]) -> Result<DilithiumKeypair> {
    check_authority(Role::User)?;
    Ok(dilithium_generate_key_pair_internal(seed))
}

/// Signs a message (Authenticated).
#[cfg(feature = "ml-dsa")]
pub fn dilithium_sign(sk: &DilithiumSigningKey, msg: &[u8], ctx: &[u8], randomness: [u8; 32]) -> Result<DilithiumSignature> {
    check_authority(Role::User)?;
    dilithium_sign_internal(sk, msg, ctx, randomness).map_err(|_| PqcError::FipsErrorState) // Map libcrux error if any
}

/// Verifies a signature (Authenticated).
#[cfg(feature = "ml-dsa")]
pub fn dilithium_verify(pk: &DilithiumVerifyingKey, msg: &[u8], ctx: &[u8], sig: &DilithiumSignature) -> Result<()> {
    check_authority(Role::User)?;
    dilithium_verify_internal(pk, msg, ctx, sig).map_err(|_| PqcError::FipsErrorState)
}

/// ML-KEM-1024 public key size in bytes.
pub const ML_KEM_1024_PK_BYTES: usize = 1568;
/// ML-KEM-1024 secret key size in bytes.
pub const ML_KEM_1024_SK_BYTES: usize = 3168;
/// ML-KEM-1024 ciphertext size in bytes.
pub const ML_KEM_1024_CT_BYTES: usize = 1568;
/// ML-KEM-1024 shared secret size in bytes.
pub const ML_KEM_1024_SS_BYTES: usize = 32;

/// ML-DSA-65 public key size in bytes.
pub const ML_DSA_65_PK_BYTES: usize = 1952;
/// ML-DSA-65 secret key size in bytes.
pub const ML_DSA_65_SK_BYTES: usize = 4032;
/// ML-DSA-65 signature size in bytes.
pub const ML_DSA_65_SIG_BYTES: usize = 3309;

pub use error::{PqcError, Result};
pub use state::{get_fips_state, is_operational, FipsState};
pub use preop::{run_post, run_post_or_panic};
pub use pct::{kyber_pct, dilithium_pct};

// CSP aliases – only one definition each
#[cfg(feature = "ml-kem")]
pub use KyberPrivateKey as KyberSecretKey;

// KyberSharedSecret is already re-exported above – do NOT re-export again
// DilithiumSigningKey is already re-exported above – do NOT re-export again