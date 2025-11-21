# FIPS 140-3 Crypto Officer & User Guide

**Module:** pqc-nostd v0.1.0

---

## 1. Installation & Initialization

### 1.1 Building the Module
To build the module in FIPS Approved mode, you must enable the `fips_140_3` feature AND run the post-build integrity injection script.

**Windows (PowerShell):**
```powershell
.\build_fips.ps1
```

**Manual Build (Other Platforms):**
1. Build the binary:
   ```bash
   cargo build --release --features "ml-kem,ml-dsa,fips_140_3"
   ```
2. Inject the HMAC:
   ```bash
   cargo run --bin inject_hmac --features "ml-kem,ml-dsa,fips_140_3" -- <path_to_binary>
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
The module automatically performs an integrity check during `run_post()`. This check verifies the HMAC-SHA-256 of the executable code against the value injected at build time.
- **Success**: `run_post()` returns `Ok(())`.
- **Failure**: `run_post()` returns `Err(PqcError::IntegrityCheckFailure)`.

The CO can also verify the integrity of the binary file on disk by re-running the `inject_hmac` tool, which will report the calculated HMAC.

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
