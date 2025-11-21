// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/ml_dsa.rs
// libcrux-ml-dsa 0.0.4 exact API

#[cfg(feature = "ml-dsa")]
pub use libcrux_ml_dsa::ml_dsa_65::portable::{generate_key_pair, sign, verify};

#[cfg(feature = "ml-dsa")]
pub use libcrux_ml_dsa::ml_dsa_65::{
    MLDSA65KeyPair as DilithiumKeypair, MLDSA65Signature as DilithiumSignature,
    MLDSA65SigningKey as DilithiumSigningKey, MLDSA65VerificationKey as DilithiumVerifyingKey,
};

/// Context string for FIPS 140-3 operations (empty for pure ML-DSA).
pub const FIPS_CONTEXT: &[u8] = b"";
