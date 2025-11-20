// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// ------------------------------------------------------------------------
// src/error.rs
/// Errors that can occur during cryptographic operations or FIPS state checks.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PqcError {
    /// A Conditional Algorithm Self-Test (CAST) failed.
    CastFailure,
    /// A Pair-wise Consistency Test (PCT) failed.
    PairwiseConsistencyTestFailure,
    /// Export of a Critical Security Parameter (CSP) was blocked.
    CspExportBlocked,
    /// The FIPS module has not been initialized.
    FipsNotInitialized,
    /// The FIPS module is currently running Power-On Self-Tests (POST).
    FipsPostInProgress,
    /// The FIPS module is in an error state.
    FipsErrorState,
    /// Authentication failed or required role not active.
    AuthenticationFailure,
    /// Software integrity check failed.
    IntegrityCheckFailure,
}

/// specialized Result type for PQC operations.
pub type Result<T> = core::result::Result<T, PqcError>;