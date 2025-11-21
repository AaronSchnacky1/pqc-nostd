// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/pct.rs – FINAL
#[cfg(any(feature = "ml-kem", feature = "ml-dsa"))]
use crate::error::{PqcError, Result};

#[cfg(feature = "ml-kem")]
use crate::{
    kyber_decapsulate_internal, kyber_encapsulate_internal, KyberPrivateKey, KyberPublicKey,
};

#[cfg(feature = "ml-dsa")]
use crate::{
    dilithium_sign_internal, dilithium_verify_internal, DilithiumSigningKey, DilithiumVerifyingKey,
    FIPS_CONTEXT,
};

/// Runs the Pair-wise Consistency Test (PCT) for ML-KEM-1024.
#[cfg(feature = "ml-kem")]
pub fn kyber_pct(pk: &KyberPublicKey, sk: &KyberPrivateKey) -> Result<()> {
    let randomness = [0x55u8; 32];
    let (ct, ss1) = kyber_encapsulate_internal(pk, randomness);
    let ss2 = kyber_decapsulate_internal(sk, &ct);
    if ss1 == ss2 {
        Ok(())
    } else {
        Err(PqcError::PairwiseConsistencyTestFailure)
    }
}

/// Runs the Pair-wise Consistency Test (PCT) for ML-DSA-65.
#[cfg(feature = "ml-dsa")]
pub fn dilithium_pct(pk: &DilithiumVerifyingKey, sk: &DilithiumSigningKey) -> Result<()> {
    let randomness = [0x77u8; 32];
    let msg = b"FIPS 140-3 PCT";
    let sig = dilithium_sign_internal(sk, msg, FIPS_CONTEXT, randomness)
        .map_err(|_| PqcError::PairwiseConsistencyTestFailure)?;
    dilithium_verify_internal(pk, msg, FIPS_CONTEXT, &sig)
        .map_err(|_| PqcError::PairwiseConsistencyTestFailure)
}
