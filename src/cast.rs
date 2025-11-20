// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/cast.rs
//! FIPS 140-3 CASTs – now compiles with digest 0.10

use crate::error::{PqcError, Result};
use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
use sha3::digest::{Update, ExtendableOutput, XofReader};  // ← critical
use hex_lit::hex;

/// Runs the SHA3-256 Conditional Algorithm Self-Test.
pub fn sha3_256_cast() -> Result<()> {
    let result = Sha3_256::digest(b"");
    if result[..] != hex!("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a") {
        Err(PqcError::CastFailure)
    } else {
        Ok(())
    }
}

/// Runs the SHA3-512 Conditional Algorithm Self-Test.
pub fn sha3_512_cast() -> Result<()> {
    let result = Sha3_512::digest(b"");
    if result[..] != hex!("a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26") {
        Err(PqcError::CastFailure)
    } else {
        Ok(())
    }
}

/// Runs the SHAKE128 Conditional Algorithm Self-Test.
pub fn shake128_cast() -> Result<()> {
    let mut h = Shake128::default();
    h.update(b"");
    let mut reader = h.finalize_xof();
    let mut out = [0u8; 32];
    reader.read(&mut out);
    if out != hex!("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26") {
        Err(PqcError::CastFailure)
    } else {
        Ok(())
    }
}

/// Runs the SHAKE256 Conditional Algorithm Self-Test.
pub fn shake256_cast() -> Result<()> {
    let mut h = Shake256::default();
    h.update(b"");
    let mut reader = h.finalize_xof();
    let mut out = [0u8; 32];
    reader.read(&mut out);
    if out != hex!("46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f") {
        Err(PqcError::CastFailure)
    } else {
        Ok(())
    }
}

/// Runs all hash-based Conditional Algorithm Self-Tests.
pub fn run_hash_casts() -> Result<()> {
    sha3_256_cast()?;
    sha3_512_cast()?;
    shake128_cast()?;
    shake256_cast()?;
    Ok(())
}