# Security Policy – pqc-nostd v0.0.2

### Approved Algorithms Only
- ML-KEM-1024 (FIPS 203) – NIST Security Level 5
- ML-DSA-65   (FIPS 204) – NIST Security Level 3

**No other cryptographic algorithms are present in the module boundary.**

### Threat Model
Designed to protect against:
- Quantum computer attacks (Shor/Grover)
- Classical cryptographic attacks
- Timing and cache side-channels (constant-time via libcrux)
- Memory safety vulnerabilities (Rust guarantees + `forbid(unsafe_code)`)

### Out of Scope
- Physical attacks
- Fault injection
- Social engineering
- Application-level key management

## FIPS 140-3 Compliance Features

| Requirement                              | Implementation                                      | Status   |
|------------------------------------------|-----------------------------------------------------|----------|
| Approved mode of operation               | `fips_140_3` feature gate                           | Complete |
| Power-On Self-Tests (POST)               | CASTs + PCTs executed in `run_post()`               | Complete |
| Conditional Algorithm Self-Tests (CASTs) | SHA-3-256/512, SHAKE-128/256 with NIST vectors      | Complete |
| Pair-wise Consistency Tests (PCTs)       | Performed on every newly generated key pair         | Complete |
| Critical Security Parameter zeroization  | `zeroize` crate on drop                             | Complete |
| Plaintext CSP export                     | Blocked in approved mode (`CspExportBlocked` error) | Complete |
| Module integrity test                    | Runtime POST only – no build.rs tricks              | Complete |
| No `std` in approved mode                | `no_std` + `no_alloc` boundary                      | Complete |
| No unsafe code                           | `forbid(unsafe_code)`                               | Complete |

## Reporting a Vulnerability

Please email vulnerabilities directly to:  
**aaronschnacky@gmail.com**

## Cryptographic Agility

The module is intentionally minimal and feature-gated. New NIST-standardized PQC algorithms can be added as optional features without affecting the existing approved boundary.

---