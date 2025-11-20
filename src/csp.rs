// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/csp.rs – FINAL
use crate::error::{PqcError, Result};
use crate::state::check_operational;

#[cfg(feature = "ml-kem")]
use crate::{KyberSecretKey, KyberSharedSecret};

#[cfg(feature = "ml-dsa")]
use crate::DilithiumSigningKey;

/// Policy for exporting Critical Security Parameters (CSPs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CspExportPolicy {
    /// Plaintext export is allowed (non-Approved mode).
    AllowPlaintext,
    /// Plaintext export is blocked (Approved mode).
    BlockPlaintext,
}

/// Returns the current CSP export policy.
pub fn get_csp_export_policy() -> CspExportPolicy {
    #[cfg(feature = "fips_140_3")]
    { CspExportPolicy::BlockPlaintext }
    #[cfg(not(feature = "fips_140_3"))]
    { CspExportPolicy::AllowPlaintext }
}

fn export_blocked() -> Result<()> {
    if matches!(get_csp_export_policy(), CspExportPolicy::BlockPlaintext) {
        Err(PqcError::CspExportBlocked)
    } else {
        Ok(())
    }
}

/// Guards the export of a Kyber secret key.
#[cfg(feature = "ml-kem")]
pub fn guard_kyber_sk_export(_sk: &KyberSecretKey) -> Result<&[u8]> {
    check_operational()?;
    export_blocked()?;
    unreachable!()
}

/// Guards the export of a Dilithium signing key.
#[cfg(feature = "ml-dsa")]
pub fn guard_dilithium_sk_export(_sk: &DilithiumSigningKey) -> Result<&[u8]> {
    check_operational()?;
    export_blocked()?;
    unreachable!()
}

/// Guards the export of a Kyber shared secret.
#[cfg(feature = "ml-kem")]
pub fn guard_shared_secret_export(_ss: &KyberSharedSecret) -> Result<&[u8]> {
    check_operational()?;
    export_blocked()?;
    unreachable!()
}