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

/// Retrieves the start address and length of the code segment (text section).
///
/// This function uses platform-specific methods to locate the executable code in memory.
///
/// # Returns
/// A tuple `(start_ptr, length)` on success.
#[allow(unsafe_code)]
pub fn get_code_segment() -> Result<(*const u8, usize)> {
    #[cfg(target_os = "windows")]
    unsafe {
        get_code_segment_windows()
    }

    #[cfg(target_os = "linux")]
    unsafe {
        get_code_segment_linux()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        // Fallback for other platforms (e.g., bare metal)
        // This would typically be defined by linker script symbols like _stext and _etext
        Err(PqcError::PlatformError)
    }
}

#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
unsafe fn get_code_segment_windows() -> Result<(*const u8, usize)> {
    // MSVC linker provides this symbol pointing to the start of the image (DOS header)
    extern "C" {
        static __ImageBase: u8;
    }

    let base_addr = &__ImageBase as *const u8;

    // Parse PE Header to find .text section or BaseOfCode
    // 1. DOS Header
    // e_magic is at offset 0. Should be 'MZ' (0x5A4D)
    let e_magic = *(base_addr as *const u16);
    if e_magic != 0x5A4D {
        return Err(PqcError::IntegrityCheckFailure);
    }

    // e_lfanew is at offset 0x3C (60). Offset to NT Headers.
    let e_lfanew = *(base_addr.add(0x3C) as *const i32);
    let nt_headers = base_addr.offset(e_lfanew as isize);

    // 2. NT Headers
    // Signature is at offset 0. Should be 'PE\0\0' (0x00004550)
    let signature = *(nt_headers as *const u32);
    if signature != 0x00004550 {
        return Err(PqcError::IntegrityCheckFailure);
    }

    // Optional Header starts after File Header.
    // File Header is 20 bytes. Signature is 4 bytes.
    // So Optional Header starts at nt_headers + 24.
    let optional_header = nt_headers.add(24);

    // Determine if PE32 or PE32+ (64-bit)
    // Magic number at offset 0 of Optional Header.
    // 0x10B = PE32, 0x20B = PE32+
    let magic = *(optional_header as *const u16);

    let (base_of_code, size_of_code) = if magic == 0x20B {
        // PE32+ (64-bit)
        // BaseOfCode is at offset 20 (0x14)
        // SizeOfCode is at offset 4 (0x04)
        let size_of_code = *(optional_header.add(4) as *const u32);
        let base_of_code = *(optional_header.add(20) as *const u32);
        (base_of_code, size_of_code)
    } else if magic == 0x10B {
        // PE32 (32-bit)
        // BaseOfCode is at offset 20 (0x14)
        // SizeOfCode is at offset 4 (0x04)
        let size_of_code = *(optional_header.add(4) as *const u32);
        let base_of_code = *(optional_header.add(20) as *const u32);
        (base_of_code, size_of_code)
    } else {
        return Err(PqcError::IntegrityCheckFailure);
    };

    let code_start = base_addr.add(base_of_code as usize);
    Ok((code_start, size_of_code as usize))
}

#[cfg(target_os = "linux")]
#[allow(unsafe_code)]
unsafe fn get_code_segment_linux() -> Result<(*const u8, usize)> {
    extern "C" {
        static __executable_start: u8;
        static _etext: u8;
    }

    let start = &__executable_start as *const u8;
    let end = &_etext as *const u8;
    let len = end as usize - start as usize;

    Ok((start, len))
}
