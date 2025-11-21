
# pqc-nostd v0.1.0

[![FIPS 140-3 CI](https://github.com/aaronschnacky1/pqc-nostd/actions/workflows/ci.yml/badge.svg)](https://github.com/aaronschnacky1/pqc-nostd/actions/workflows/ci.yml)
![Pure Rust](https://img.shields.io/badge/100%25-Rust-orange)
![no_std](https://img.shields.io/badge/no__std-Ready-green)
[![Crates.io](https://img.shields.io/crates/v/pqc-nostd.svg)](https://crates.io/crates/pqc-nostd)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Implements only NIST-standardized and FIPS-approved algorithms:
- ML-KEM-1024 (FIPS 203 – CRYSTALS-Kyber) – Security Level 5
- ML-DSA-65   (FIPS 204 – CRYSTALS-Dilithium) – Security Level 3

No `std`, no heap allocation, no unsafe code, no legacy crypto – ever.

### FIPS 140-3 Compliance Status (20 November 2025)
- Designed and for **Level 2** validation (pure software, no_std boundary)
- Fully satisfies **Level 1** requirements
- Complete Power-On Self-Tests (POST):
  - Conditional Algorithm Self-Tests (CASTs) on all SHA-3/SHAKE instances
  - Pair-wise Consistency Tests (PCTs) on every newly generated key pair
- Critical Security Parameters automatically zeroized on drop
- Plaintext CSP export blocked in approved mode
- Approved mode enforced via `fips_140_3` feature gate
- Zero dependencies that require `std` in approved mode

### Cargo Features
| Feature         | Description                                    | Required for FIPS |
|-----------------|--------------------------------------------------------|-------------------|
| `ml-kem`        | Enables ML-KEM-1024 (FIPS 203)                          | Yes               |
| `ml-dsa`        | Enables ML-DSA-65 (FIPS 204)                            | Yes               |
| `fips_140_3`    | Approved mode – POST, CSP controls, CASTs, operational state machine | Yes               |

No default features – the FIPS boundary is explicit and minimal.

### Approved-Mode Usage Example
```rust
use pqc_nostd::{run_post_or_panic, is_operational, FIPS_CONTEXT};

fn main() {
    run_post_or_panic();  // Executes full POST – panics on any failure
    assert!(is_operational());

    // ML-KEM-1024 (Kyber)
    let kyber_kp = pqc_nostd::kyber_generate_key_pair([0x01u8; 64]);
    let (ct, ss_alice) = pqc_nostd::encapsulate(&kyber_kp.public_key(), [0x02u8; 32]);
    let ss_bob = pqc_nostd::decapsulate(&kyber_kp.private_key(), &ct);
    assert_eq!(ss_alice, ss_bob);

    // ML-DSA-65 (Dilithium)
    let dil_kp = pqc_nostd::dilithium_generate_key_pair([0x03u8; 32]);
    let sig = pqc_nostd::dilithium_sign(&dil_kp.signing_key, b"msg", FIPS_CONTEXT, [0x04u8; 32]).unwrap();
    assert!(pqc_nostd::dilithium_verify(&dil_kp.verification_key, b"msg", FIPS_CONTEXT, &sig).is_ok());
}