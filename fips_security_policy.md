# FIPS 140-3 Security Policy

**Module Name:** pqc-nostd
**Module Version:** v0.1.0
**Security Level:** 2

---

## 1. Introduction
This document is the non-proprietary Security Policy for the `pqc-nostd` cryptographic module. It describes how the module meets the security requirements of FIPS 140-3 Level 2.

## 2. Cryptographic Module Specification
The `pqc-nostd` module is a **Software** module (Security Level 2) running on a general-purpose computer.
- **Physical Boundary:** The physical enclosure of the host platform (e.g., PC case).
- **Logical Boundary:** The binary image of the `pqc-nostd` crate.

### 2.1 Approved Algorithms
The module implements the following NIST-Approved algorithms:
| Algorithm | Standard | Usage |
|-----------|----------|-------|
| **ML-KEM-1024** | FIPS 203 | Key Encapsulation Mechanism |
| **ML-DSA-65** | FIPS 204 | Digital Signature |
| **SHA-3** | FIPS 202 | Hashing (Internal use) |
| **HMAC-SHA-256**| FIPS 198 | Software Integrity Test |

## 3. Cryptographic Module Ports and Interfaces
The module provides a logical interface via its Rust API.
- **Data Input:** Function arguments (`msg`, `ct`, `pk`).
- **Data Output:** Function return values (`sig`, `ss`, `ct`).
- **Control Input:** Function calls (`run_post`, `login`).
- **Status Output:** Return types (`Result<()>`, `FipsState`).

## 4. Roles, Services, and Authentication
The module supports Level 2 Role-Based Authentication.

### 4.1 Roles
| Role | Description |
|------|-------------|
| **User** | Performs cryptographic operations (Sign, Verify, Encapsulate, Decapsulate). |
| **Crypto Officer (CO)** | Performs module management (POST, Integrity Check, Zeroization). |

### 4.2 Authentication
- **Type:** Role-Based.
- **Mechanism:** Password-based (demonstration).
- **Strength:** The module requires explicit login via `pqc_nostd::auth::login(role, password)`.

### 4.3 Services
| Service | Role | Description |
|---------|------|-------------|
| `run_post` | CO, User | Runs Power-On Self-Tests. |
| `integrity_check` | CO | Verifies software integrity. |
| `login` | Unauth | Authenticates an operator. |
| `logout` | CO, User | Logs out the current operator. |
| `encapsulate` | User | ML-KEM Encapsulation. |
| `decapsulate` | User | ML-KEM Decapsulation. |
| `sign` | User | ML-DSA Signing. |
| `verify` | User | ML-DSA Verification. |
| `keygen` | User | Key Generation (ML-KEM, ML-DSA). |

## 5. Physical Security
The module is software-only and relies on the physical security of the host platform.

## 6. Operational Environment
The module operates in a modifiable operational environment. The operating system must be configured to single-user mode or restrict access to the module to a single operator at a time.

## 7. Self-Tests
The module performs the following self-tests:
- **Power-On Self-Tests (POST):**
    - **Software Integrity Test**: HMAC-SHA-256 of the code segment.
    - **Known Answer Tests (KATs)**:
        - ML-KEM-1024: Key Generation, Encapsulation, Decapsulation.
        - ML-DSA-65: Key Generation, Signing, Verification.
    - **Conditional Algorithm Self-Tests (CASTs)**:
        - SHA-3-256, SHA-3-512.
        - SHAKE128, SHAKE256.
    - **Pair-wise Consistency Tests (PCTs)**:
        - ML-KEM-1024 (Round-trip).
        - ML-DSA-65 (Sign/Verify).
- **Conditional Tests:**
    - Pair-wise Consistency Test (PCT) on every key generation.

## 8. Mitigation of Other Attacks
The module implements constant-time logic (via `libcrux`) to mitigate timing side-channel attacks.
