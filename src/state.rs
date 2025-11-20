// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/state.rs
use crate::error::{PqcError, Result};
use core::sync::atomic::{AtomicU8, Ordering};

/// Represents the current state of the FIPS 140-3 module.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FipsState {
    /// Module has not started yet.
    Uninitialized = 0,
    /// Power-On Self-Tests are running.
    POST = 1,
    /// Module is in Approved mode and fully operational.
    Operational = 2,
    /// Module is in an error state (soft or hard error).
    Error = 3,
}

static FIPS_STATE: AtomicU8 = AtomicU8::new(FipsState::Uninitialized as u8);

/// Returns the current FIPS state.
pub fn get_fips_state() -> FipsState {
    FipsState::from(FIPS_STATE.load(Ordering::Acquire))
}

/// Checks if the module is in the Operational state.
pub fn is_operational() -> bool {
    get_fips_state() == FipsState::Operational
}

/// Returns Ok(()) if operational, otherwise returns the specific error state.
pub fn check_operational() -> Result<()> {
    match get_fips_state() {
        FipsState::Operational => Ok(()),
        FipsState::Uninitialized => Err(PqcError::FipsNotInitialized),
        FipsState::POST => Err(PqcError::FipsPostInProgress),
        FipsState::Error => Err(PqcError::FipsErrorState),
    }
}

/// Resets the FIPS state to Uninitialized.
pub fn reset_fips_state() {
    FIPS_STATE.store(FipsState::Uninitialized as u8, Ordering::Release);
}

pub(crate) fn enter_post_state() {
    FIPS_STATE.store(FipsState::POST as u8, Ordering::Release);
}

pub(crate) fn enter_operational_state() {
    FIPS_STATE.store(FipsState::Operational as u8, Ordering::Release);
}

pub(crate) fn enter_error_state() {
    FIPS_STATE.store(FipsState::Error as u8, Ordering::Release);
}

impl From<u8> for FipsState {
    fn from(val: u8) -> Self {
        match val {
            0 => FipsState::Uninitialized,
            1 => FipsState::POST,
            2 => FipsState::Operational,
            _ => FipsState::Error,
        }
    }
}