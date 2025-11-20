// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/integrity.rs
//! Software Integrity Test (Section 6.10.1).

use crate::error::{PqcError, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Performs a software integrity check using HMAC-SHA-256.
///
/// # Arguments
/// * `code_start` - Pointer to the start of the code segment.
/// * `code_len` - Length of the code segment in bytes.
/// * `expected_hmac` - The expected HMAC-SHA-256 tag (32 bytes).
///
/// # Safety
/// This function is unsafe because it reads raw memory specified by `code_start` and `code_len`.
/// The caller must ensure these bounds are valid and point to the actual code segment.
#[allow(unsafe_code)]
pub unsafe fn integrity_check(
    code_start: *const u8,
    code_len: usize,
    expected_hmac: &[u8],
) -> Result<()> {
    let code_slice = core::slice::from_raw_parts(code_start, code_len);
    
    // Key for HMAC-SHA-256 Integrity Test
    // In a real module, this key would be embedded or derived securely.
    // For this demonstration, we use a hardcoded key.
    let integrity_key = b"FIPS_140_3_INTEGRITY_KEY";

    let mut mac = HmacSha256::new_from_slice(integrity_key)
        .map_err(|_| PqcError::IntegrityCheckFailure)?;
    
    mac.update(code_slice);
    
    if mac.verify_slice(expected_hmac).is_ok() {
        Ok(())
    } else {
        Err(PqcError::IntegrityCheckFailure)
    }
}
