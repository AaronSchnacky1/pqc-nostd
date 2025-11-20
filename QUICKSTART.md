# FIPS 140-3 Completion - Quick Start Guide

This guide provides the immediate next steps to complete FIPS 140-3 compliance.

## 📋 Current Status

- ✅ ML-KEM-1024 KeyGen golden values (complete)
- ❌ ML-KEM-1024 Encapsulation/Decapsulation values (placeholders)
- ❌ ML-DSA-65 KeyGen golden values (placeholders)
- ❌ ML-DSA-65 Sign/Verify values (placeholders)
- ❌ Integrity check integration (not in POST)
- ❌ Performance benchmarks (don't exist)
- ⚠️ CMVP documentation (partial)

## 🚀 Phase 1: Generate KAT Golden Values (START HERE)

### Step 1: Run the KAT Generator

```powershell
# Navigate to project root
cd C:\Users\aaron\OneDrive\Desktop\pqc-nostd

# Run the generator script
cargo run --features "ml-kem,ml-dsa,fips_140_3" --bin generate_kat_values > kat_output.txt
```

**Note**: This will fail initially because the script needs to be added to Cargo.toml

### Step 2: Add Generator to Cargo.toml

Add this to `Cargo.toml`:

```toml
[[bin]]
name = "generate_kat_values"
path = "scripts/generate_kat_values.rs"
```

### Step 3: Generate the Values

```powershell
cargo run --features "ml-kem,ml-dsa,fips_140_3" --bin generate_kat_values
```

This will output all the golden values in chunks, formatted for easy copying.

### Step 4: Update src/kat.rs

Copy each section from the output and replace the placeholders in `src/kat.rs`:

1. **Lines 371-374**: ML-KEM Ciphertext (4 chunks)
2. **Lines 377-380**: ML-KEM Shared Secret (1 chunk)
3. **Lines 405-408**: ML-DSA Verification Key (4 chunks)
4. **Lines 411-414**: ML-DSA Signing Key (8 chunks)
5. **Lines 430-433**: ML-DSA Signature (6 chunks)

### Step 5: Integrate KATs into POST

Edit `src/preop.rs`, add after line 23 (after `run_hash_casts()?;`):

```rust
// Run Known Answer Tests
#[cfg(feature = "fips_140_3")]
crate::kat::run_kats()?;
```

### Step 6: Test

```powershell
cargo test --features "ml-kem,ml-dsa,fips_140_3"
```

**Expected**: All tests pass ✅

## 🔒 Phase 2: Integrate Integrity Check

### Step 1: Add build.rs Support

Create `build.rs` in project root (this is complex - see detailed plan).

### Step 2: Update preop.rs

Add integrity check to POST (after KATs).

### Step 3: Test

Verify integrity check works on clean build.

## 📊 Phase 3: Add Benchmarks

### Step 1: Add Criterion Dependency

Add to `Cargo.toml`:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "crypto_benchmarks"
harness = false
```

### Step 2: Create Benchmark File

Create `benches/crypto_benchmarks.rs` (see detailed plan for content).

### Step 3: Run Benchmarks

```powershell
cargo bench --features "ml-kem,ml-dsa,fips_140_3"
```

## 📝 Phase 4: CMVP Documentation

### Step 1: Enhance Security Policy

Update `fips_security_policy.md` with detailed information.

### Step 2: Create Test Evidence

Run tests and capture output for submission.

### Step 3: Create Additional Documentation

- Finite State Model
- Vendor Information
- Architecture Documentation

## ✅ Phase 5: Final Validation

### Step 1: Run All Tests

```powershell
# All features
cargo test --all-features

# FIPS only
cargo test --no-default-features --features "ml-kem,ml-dsa,fips_140_3"

# Individual algorithms
cargo test --features ml-kem
cargo test --features ml-dsa
```

### Step 2: Code Quality

```powershell
# Check for warnings
cargo clippy --all-features

# Format code
cargo fmt

# Check documentation
cargo doc --all-features --no-deps
```

### Step 3: Build Release

```powershell
cargo build --release --features "ml-kem,ml-dsa,fips_140_3"
```

## 📅 Recommended Timeline

- **Day 1-2**: Phase 1 (KAT values) ← START HERE
- **Day 3-4**: Phase 2 (Integrity check)
- **Day 5**: Phase 3 (Benchmarks)
- **Week 2**: Phase 4 (CMVP docs)
- **Week 3**: Phase 5 (Final validation)

## 🆘 Troubleshooting

### Issue: KAT Generator Won't Compile

**Solution**: Make sure you've added the `[[bin]]` section to Cargo.toml

### Issue: KAT Tests Fail

**Solution**: 
1. Verify you copied the golden values correctly
2. Check for any trailing commas or formatting issues
3. Ensure you're using the exact same seeds as the generator

### Issue: Integrity Check Too Complex

**Solution**: Start with a simpler build-time approach, document limitations

## 📚 Reference Documents

- Full Plan: `.agent/workflows/fips-completion-plan.md`
- FIPS 140-3 Standard: https://csrc.nist.gov/publications/detail/fips/140/3/final
- FIPS 140-3 IG: https://csrc.nist.gov/projects/fips-140-3-transition-effort

## 🎯 Success Metrics

After completing all phases:

- [ ] All KAT golden values populated (no placeholders)
- [ ] KATs integrated into POST
- [ ] All tests pass
- [ ] Integrity check integrated
- [ ] Benchmarks running
- [ ] CMVP documentation complete
- [ ] Ready for submission

---

**Next Action**: Run the KAT generator and start updating kat.rs!
