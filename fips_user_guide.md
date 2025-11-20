# FIPS 140-3 Crypto Officer & User Guide

**Module:** pqc-nostd v0.0.2

---

## 1. Installation & Initialization

### 1.1 Building the Module
To build the module in FIPS Approved mode, you must enable the `fips_140_3` feature:

```bash
cargo build --release --features "ml-kem,ml-dsa,fips_140_3"
```

### 1.2 Power-Up Procedures
Upon power-up, the application **MUST** call `run_post()` or `run_post_or_panic()` immediately. No cryptographic operations are permitted until this function returns successfully.

```rust
use pqc_nostd::run_post_or_panic;

fn main() {
    // 1. Execute Self-Tests
    run_post_or_panic();
    
    // 2. Proceed with application logic...
}
```

## 2. Crypto Officer Guidance

### 2.1 Responsibilities
The Crypto Officer (CO) is responsible for:
- Installing the module.
- Verifying the integrity of the module.
- Managing the operational environment.

### 2.2 Integrity Check
The CO should verify the module's integrity using the provided `integrity_check` function (if exposed) or by verifying the HMAC of the binary externally before execution.

## 3. User Guidance

### 3.1 Authentication (Level 2)
Before performing any cryptographic operation, the User must authenticate.

```rust
use pqc_nostd::auth::{login, Role};

// Authenticate as User
login(Role::User, b"user123").expect("Authentication failed");
```

### 3.2 Key Generation
Use the Approved key generation functions. Ensure the random seed is from an Approved DRBG (not provided by this module).

```rust
let seed = [0u8; 64]; // MUST be from Approved DRBG
let kp = pqc_nostd::kyber_generate_key_pair(seed).unwrap();
```

### 3.3 Zeroization
Sensitive keys are automatically zeroized when they go out of scope (via the `Drop` trait). The User must ensure that variables containing keys are dropped when no longer needed.

## 4. Error States
If the module enters the `Error` state (e.g., due to a self-test failure), all cryptographic operations will return `Err(PqcError::FipsErrorState)`. The User must restart the module (power cycle) to recover.
