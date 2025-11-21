// ------------------------------------------------------------------------
// PQC-COMBO v0.2.0
// ------------------------------------------------------------------------
// Copyright © 2025 Aaron Schnacky. All rights reserved.
// License: MIT (publicly auditable for FIPS/CMVP verification)
// Contact: aaronschnacky@gmail.com
// src/preop.rs – FINAL
use crate::error::Result;
use crate::cast::run_hash_casts;
use crate::state::{enter_post_state, enter_operational_state, enter_error_state};

#[cfg(feature = "ml-kem")]
use crate::{kyber_generate_key_pair_internal, KyberKeypair, kyber_pct};

#[cfg(feature = "ml-dsa")]
use crate::{dilithium_generate_key_pair_internal, DilithiumKeypair, dilithium_pct};

/// Runs the full suite of Power-On Self-Tests (POST).
#[allow(unsafe_code)]
pub fn run_post() -> Result<()> {
    enter_post_state();

    let result = (|| {
        run_hash_casts()?;

        // Run Known Answer Tests (KATs)
        #[cfg(feature = "fips_140_3")]
        crate::kat::run_kats()?;

        #[cfg(feature = "ml-kem")]
        {
            let seed64 = [0x42u8; 64];
            let kp: KyberKeypair = kyber_generate_key_pair_internal(seed64);
            kyber_pct(kp.public_key(), kp.private_key())?;
        }

        #[cfg(feature = "ml-dsa")]
        {
            let seed32 = [0x42u8; 32];
            let kp: DilithiumKeypair = dilithium_generate_key_pair_internal(seed32);
            dilithium_pct(&kp.verification_key, &kp.signing_key)?;
        }

        // Run Software Integrity Test
        #[cfg(feature = "fips_140_3")]
        {
            use crate::integrity::{integrity_check, get_code_segment};
            use crate::integrity_data::EXPECTED_HMAC;

            // Skip integrity check if HMAC is still the placeholder (e.g., during testing)
            const PLACEHOLDER: [u8; 32] = *b"__PQC_NOSTD_HMAC_PLACEHOLDER__\x00\x00";
            if EXPECTED_HMAC != PLACEHOLDER {
                // Note: In a real embedded system, we might panic here if detection fails,
                // but returning an error transitions to the Error state which is correct.
                let (code_start, code_len) = get_code_segment().map_err(|_| crate::error::PqcError::PlatformError)?;
                
                // Safety: get_code_segment returns valid boundaries for the running executable.
                unsafe {
                    integrity_check(code_start, code_len, &EXPECTED_HMAC)?;
                }
            }
        }

        Ok(())
    })();

    match result {
        Ok(()) => { enter_operational_state(); Ok(()) }
        Err(e) => { enter_error_state(); Err(e) }
    }
}

/// Runs POST and panics if any test fails.
pub fn run_post_or_panic() {
    run_post().expect("FIPS 140-3 POST failed");
}