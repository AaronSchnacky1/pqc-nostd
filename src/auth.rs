// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/auth.rs
//! Role-Based Authentication (Level 2 Requirement).

use crate::error::{PqcError, Result};
use core::sync::atomic::{AtomicU8, Ordering};

/// FIPS 140-3 Roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// User role (can perform crypto operations).
    User,
    /// Crypto Officer role (can perform management/integrity functions).
    CryptoOfficer,
}

/// Authentication state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthState {
    LoggedOut = 0,
    LoggedInUser = 1,
    LoggedInCO = 2,
}

static AUTH_STATE: AtomicU8 = AtomicU8::new(AuthState::LoggedOut as u8);

/// Logs in with the specified role and password.
/// 
/// **Note:** In a real module, passwords would be hashed and compared against stored hashes.
/// For this demonstration, we use simple hardcoded checks.
pub fn login(role: Role, password: &[u8]) -> Result<()> {
    // Mock credentials
    let user_pw = b"user123";
    let co_pw = b"admin456";

    match role {
        Role::User => {
            if password == user_pw {
                AUTH_STATE.store(AuthState::LoggedInUser as u8, Ordering::Release);
                Ok(())
            } else {
                Err(PqcError::AuthenticationFailure)
            }
        }
        Role::CryptoOfficer => {
            if password == co_pw {
                AUTH_STATE.store(AuthState::LoggedInCO as u8, Ordering::Release);
                Ok(())
            } else {
                Err(PqcError::AuthenticationFailure)
            }
        }
    }
}

/// Logs out the current operator.
pub fn logout() {
    AUTH_STATE.store(AuthState::LoggedOut as u8, Ordering::Release);
}

/// Checks if the current operator has the required authority.
pub fn check_authority(required_role: Role) -> Result<()> {
    let current = AUTH_STATE.load(Ordering::Acquire);
    match (required_role, current) {
        (Role::User, 1) => Ok(()), // User is logged in
        (Role::CryptoOfficer, 2) => Ok(()), // CO is logged in
        _ => Err(PqcError::AuthenticationFailure),
    }
}

/// Returns true if any user is logged in.
pub fn is_authenticated() -> bool {
    AUTH_STATE.load(Ordering::Acquire) != AuthState::LoggedOut as u8
}
