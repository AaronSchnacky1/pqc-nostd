// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/ml_kem.rs
// libcrux-ml-kem 0.0.4 exact API

#[cfg(feature = "ml-kem")]
pub use libcrux_ml_kem::mlkem1024::portable::{
    encapsulate,
    decapsulate,
    generate_key_pair,
};

#[cfg(feature = "ml-kem")]
pub use libcrux_ml_kem::mlkem1024::{
    MlKem1024Ciphertext as KyberCiphertext,
    MlKem1024KeyPair as KyberKeypair,
    MlKem1024PublicKey as KyberPublicKey,
    MlKem1024PrivateKey as KyberPrivateKey,
};

/// ML-KEM-1024 shared secret type (32 bytes).
pub type KyberSharedSecret = [u8; 32];