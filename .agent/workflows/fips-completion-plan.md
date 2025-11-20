---
description: Complete FIPS 140-3 Compliance Implementation Plan
---

# FIPS 140-3 Compliance Completion Plan

**Project**: pqc-nostd v0.0.2  
**Goal**: Achieve full FIPS 140-3 Level 2 compliance and prepare for CMVP submission  
**Created**: 2025-11-20

---

## Phase 1: Complete Known Answer Tests (KATs) ⚠️ HIGH PRIORITY

**Objective**: Replace all placeholder KAT values with actual golden values

### Task 1.1: Generate ML-KEM Golden Values
**Estimated Time**: 2-3 hours

#### Subtasks:
1. **Generate ML-KEM Encapsulation Ciphertext** (1568 bytes)
   - Run: `kyber_encapsulate_internal()` with seed `[0xAAu8; 64]` and randomness `[0xBBu8; 32]`
   - Capture ciphertext output
   - Split into 4 chunks of ~392 bytes each for readability
   - Update `src/kat.rs` lines 371-374

2. **Generate ML-KEM Shared Secret** (32 bytes)
   - Capture shared secret from same encapsulation
   - Single chunk (small enough)
   - Update `src/kat.rs` lines 377-380

**Files to modify**: `src/kat.rs` (lines 371-380)

---

### Task 1.2: Generate ML-DSA Golden Values
**Estimated Time**: 3-4 hours

#### Subtasks:
1. **Generate ML-DSA Verification Key** (1952 bytes)
   - Run: `dilithium_generate_key_pair_internal()` with seed `[0xCCu8; 32]`
   - Capture verification key output
   - Split into 4 chunks of ~488 bytes each
   - Update `src/kat.rs` lines 405-408

2. **Generate ML-DSA Signing Key** (4032 bytes)
   - Capture signing key from same key generation
   - Split into 8 chunks of ~504 bytes each (larger key)
   - Update `src/kat.rs` lines 411-414

3. **Generate ML-DSA Signature** (3309 bytes)
   - Run: `dilithium_sign_internal()` with message `b"FIPS 140-3 KAT"` and randomness `[0xDDu8; 32]`
   - Capture signature output
   - Split into 6 chunks of ~551 bytes each
   - Update `src/kat.rs` lines 430-433

**Files to modify**: `src/kat.rs` (lines 405-433)

---

### Task 1.3: Integrate KATs into POST
**Estimated Time**: 30 minutes

#### Subtasks:
1. Add `run_kats()` call to `run_post()` function
2. Ensure KATs run after CASTs but before PCTs
3. Test that POST fails if KATs fail
4. Verify error state is entered on KAT failure

**Files to modify**: `src/preop.rs`

---

### Task 1.4: Test KAT Implementation
**Estimated Time**: 1 hour

#### Subtasks:
1. Run full test suite with KATs enabled
2. Verify all KATs pass with correct golden values
3. Test that modified values cause KAT failures
4. Document KAT test results

**Command**: `cargo test --features "ml-kem,ml-dsa,fips_140_3"`

---

## Phase 2: Software Integrity Test Integration ⚠️ HIGH PRIORITY

**Objective**: Integrate integrity check into POST with platform-specific values

### Task 2.1: Determine Platform-Specific Code Boundaries
**Estimated Time**: 2-3 hours

#### Subtasks:
1. **Research platform-specific approaches**:
   - Windows: Use PE header parsing or linker symbols
   - Linux: Use ELF sections or `/proc/self/maps`
   - Embedded: Use linker script symbols

2. **Implement platform detection**:
   - Add conditional compilation for different platforms
   - Create helper functions to get code boundaries
   - Handle `no_std` constraint (may need build-time approach)

3. **Design build-time integrity approach**:
   - Consider using `build.rs` to compute HMAC at build time
   - Store expected HMAC as a constant
   - Document that integrity check is build-specific

**Files to create/modify**: 
- `src/integrity.rs` (add platform-specific helpers)
- `build.rs` (new file for build-time HMAC computation)

---

### Task 2.2: Implement Build-Time HMAC Generation
**Estimated Time**: 3-4 hours

#### Subtasks:
1. Create `build.rs` script
2. Compute HMAC-SHA-256 of compiled binary
3. Generate Rust code with expected HMAC constant
4. Include generated code in build
5. Handle incremental builds (HMAC changes each build)

**Files to create**: `build.rs`

---

### Task 2.3: Integrate Integrity Check into POST
**Estimated Time**: 1-2 hours

#### Subtasks:
1. Add `integrity_check()` call to `run_post()`
2. Place before or after CASTs (document decision)
3. Pass platform-specific `code_start` and `code_len`
4. Pass build-time generated `expected_hmac`
5. Handle errors appropriately

**Files to modify**: `src/preop.rs`

---

### Task 2.4: Test Integrity Check
**Estimated Time**: 2 hours

#### Subtasks:
1. Verify integrity check passes on clean build
2. Test that modified binary fails integrity check
3. Test on multiple platforms (Windows, Linux if possible)
4. Document platform-specific behavior

**Note**: This may be challenging in `no_std` environment. May need to document limitations.

---

## Phase 3: Performance Benchmarking 🟢 LOWER PRIORITY

**Objective**: Create comprehensive benchmark suite

### Task 3.1: Set Up Benchmark Infrastructure
**Estimated Time**: 1 hour

#### Subtasks:
1. Add `criterion` to `[dev-dependencies]`
2. Create `benches/` directory
3. Create `benches/crypto_benchmarks.rs`
4. Configure benchmark harness in `Cargo.toml`

**Files to create**:
- `benches/crypto_benchmarks.rs`

**Files to modify**:
- `Cargo.toml` (add dev-dependencies)

---

### Task 3.2: Implement Algorithm Benchmarks
**Estimated Time**: 2-3 hours

#### Subtasks:
1. **ML-KEM-1024 benchmarks**:
   - Key generation
   - Encapsulation
   - Decapsulation
   - Full round-trip

2. **ML-DSA-65 benchmarks**:
   - Key generation
   - Signing (various message sizes)
   - Verification (various message sizes)
   - Full sign-verify cycle

3. **Self-Test benchmarks**:
   - Individual CASTs (SHA-3-256, SHA-3-512, SHAKE-128, SHAKE-256)
   - Individual PCTs (ML-KEM, ML-DSA)
   - Full POST execution time
   - KATs execution time

**Files to modify**: `benches/crypto_benchmarks.rs`

---

### Task 3.3: Document Performance Results
**Estimated Time**: 1 hour

#### Subtasks:
1. Run benchmarks on reference hardware
2. Create performance documentation
3. Add results to README or separate PERFORMANCE.md
4. Include in FIPS documentation

**Files to create**: `PERFORMANCE.md`

---

## Phase 4: CMVP Submission Preparation 🟡 MEDIUM PRIORITY

**Objective**: Prepare complete CMVP submission package

### Task 4.1: Algorithm Validation (CAVP)
**Estimated Time**: Variable (weeks to months)

#### Subtasks:
1. **Identify CAVP testing requirements**:
   - ML-KEM-1024 (FIPS 203)
   - ML-DSA-65 (FIPS 204)
   - SHA-3 family (FIPS 202)
   - HMAC-SHA-256 (FIPS 198)

2. **Generate CAVP test vectors**:
   - Download NIST test vector files
   - Implement CAVP test harness
   - Generate responses for all test vectors

3. **Submit to CAVP**:
   - Find accredited CAVP lab or use NIST ACVTS
   - Submit algorithm implementations
   - Obtain CAVP certificates

**Note**: This is typically done through an accredited lab and can take months.

---

### Task 4.2: Complete Security Policy Documentation
**Estimated Time**: 4-6 hours

#### Subtasks:
1. **Enhance `fips_security_policy.md`**:
   - Add detailed module specification
   - Document all cryptographic boundaries
   - Add detailed operational environment description
   - Include complete self-test descriptions
   - Add mitigation of other attacks section

2. **Add required sections**:
   - Physical security (even for software modules)
   - EMI/EMC (if applicable)
   - Self-test details with expected results
   - Guidance and secure operation

**Files to modify**: `fips_security_policy.md`

---

### Task 4.3: Create Test Evidence Package
**Estimated Time**: 3-4 hours

#### Subtasks:
1. **Document all test results**:
   - POST test logs
   - CAST test results with expected values
   - PCT test results
   - KAT test results
   - Integration test results

2. **Create test evidence files**:
   - `docs/test_evidence/post_results.txt`
   - `docs/test_evidence/cast_results.txt`
   - `docs/test_evidence/pct_results.txt`
   - `docs/test_evidence/kat_results.txt`

3. **Automate test evidence generation**:
   - Create script to run all tests and capture output
   - Format output for CMVP submission

**Files to create**: 
- `docs/test_evidence/` directory
- `scripts/generate_test_evidence.sh` or `.ps1`

---

### Task 4.4: Create Finite State Model Documentation
**Estimated Time**: 2-3 hours

#### Subtasks:
1. Document state machine formally
2. Create state transition diagrams
3. Document all state transitions and conditions
4. Include error handling states

**Files to create**: `docs/finite_state_model.md`

---

### Task 4.5: Prepare Vendor Information
**Estimated Time**: 1-2 hours

#### Subtasks:
1. Create vendor information sheet
2. Document module version and build information
3. Create module identification documentation
4. Prepare contact information

**Files to create**: `docs/vendor_information.md`

---

### Task 4.6: Source Code Documentation
**Estimated Time**: 2-3 hours

#### Subtasks:
1. Ensure all code is well-commented
2. Create architecture documentation
3. Document cryptographic boundaries in code
4. Create code review checklist

**Files to create**: `docs/architecture.md`, `docs/code_review_checklist.md`

---

### Task 4.7: Compile CMVP Submission Package
**Estimated Time**: 2-3 hours

#### Subtasks:
1. Gather all required documents
2. Create submission package structure
3. Write cover letter/executive summary
4. Review completeness against FIPS 140-3 IG

**Files to create**: `docs/cmvp_submission/` directory with all materials

---

## Phase 5: Final Testing and Validation ✅ FINAL PHASE

**Objective**: Comprehensive testing before submission

### Task 5.1: Full Integration Testing
**Estimated Time**: 2-3 hours

#### Subtasks:
1. Run all tests with all features
2. Test on multiple platforms
3. Test error conditions
4. Verify state machine behavior

**Commands**:
```bash
cargo test --all-features
cargo test --no-default-features --features "ml-kem,ml-dsa,fips_140_3"
cargo test --features ml-kem
cargo test --features ml-dsa
```

---

### Task 5.2: Code Quality Review
**Estimated Time**: 2 hours

#### Subtasks:
1. Run `cargo clippy` and fix all warnings
2. Run `cargo fmt` to ensure consistent formatting
3. Review all `unsafe` code (should only be in integrity check)
4. Verify `#![deny(unsafe_code)]` is still in place (except where needed)

---

### Task 5.3: Documentation Review
**Estimated Time**: 2 hours

#### Subtasks:
1. Review all documentation for accuracy
2. Ensure all public APIs are documented
3. Update README with latest information
4. Create CHANGELOG entry for completion

---

## Summary Timeline

| Phase | Estimated Time | Priority |
|-------|----------------|----------|
| **Phase 1: KATs** | 7-9 hours | 🔴 HIGH |
| **Phase 2: Integrity** | 8-11 hours | 🔴 HIGH |
| **Phase 3: Benchmarks** | 4-5 hours | 🟢 LOW |
| **Phase 4: CMVP Prep** | 16-23 hours + CAVP time | 🟡 MEDIUM |
| **Phase 5: Final Testing** | 6-7 hours | ✅ FINAL |
| **TOTAL** | **41-55 hours** + CAVP validation time |

---

## Recommended Execution Order

### Week 1: Critical Functionality
1. ✅ Task 1.1: Generate ML-KEM golden values
2. ✅ Task 1.2: Generate ML-DSA golden values
3. ✅ Task 1.3: Integrate KATs into POST
4. ✅ Task 1.4: Test KAT implementation

### Week 2: Integrity Check
5. ✅ Task 2.1: Platform-specific code boundaries
6. ✅ Task 2.2: Build-time HMAC generation
7. ✅ Task 2.3: Integrate into POST
8. ✅ Task 2.4: Test integrity check

### Week 3: Documentation & Testing
9. ✅ Task 4.2: Complete security policy
10. ✅ Task 4.3: Test evidence package
11. ✅ Task 4.4: Finite state model
12. ✅ Task 5.1-5.3: Final testing and review

### Week 4: Benchmarks & CMVP Prep
13. ✅ Task 3.1-3.3: Performance benchmarking
14. ✅ Task 4.5-4.7: CMVP submission package

### Ongoing: CAVP Validation
- Task 4.1: Work with accredited lab (parallel to other work)

---

## Success Criteria

### Phase 1 Complete When:
- [ ] All KAT golden values are populated (no `0x00` placeholders)
- [ ] KATs are integrated into POST
- [ ] All tests pass with `cargo test --all-features`
- [ ] KATs fail when golden values are modified (negative test)

### Phase 2 Complete When:
- [ ] Integrity check is integrated into POST
- [ ] Build-time HMAC generation works
- [ ] Integrity check passes on clean build
- [ ] Integrity check fails on modified binary
- [ ] Platform-specific implementation documented

### Phase 3 Complete When:
- [ ] All benchmarks run successfully
- [ ] Performance results documented
- [ ] Benchmarks included in CI/CD (optional)

### Phase 4 Complete When:
- [ ] All CMVP documentation complete
- [ ] CAVP certificates obtained (or in progress)
- [ ] Test evidence package complete
- [ ] Submission package ready for review

### Phase 5 Complete When:
- [ ] All tests pass on all platforms
- [ ] No clippy warnings
- [ ] All documentation reviewed and updated
- [ ] Ready for CMVP submission

---

## Risk Mitigation

### Risk 1: Platform-Specific Integrity Check Complexity
**Mitigation**: 
- Start with build-time approach (simpler)
- Document platform limitations
- Consider runtime approach as enhancement

### Risk 2: CAVP Validation Timeline
**Mitigation**:
- Start CAVP process early (parallel work)
- Use accredited lab to speed up process
- Consider ACVTS for automated testing

### Risk 3: KAT Value Generation Errors
**Mitigation**:
- Generate values multiple times to verify consistency
- Cross-check with NIST test vectors where available
- Implement negative tests (modified values should fail)

### Risk 4: `no_std` Constraints
**Mitigation**:
- Design solutions that work in `no_std` environment
- Document any limitations
- Consider conditional compilation for `std` features in tests

---

## Notes

- All work should maintain `no_std` compatibility
- All work should maintain `#![deny(unsafe_code)]` except where absolutely necessary
- All changes should be tested on Windows (primary platform)
- Consider Linux testing for portability
- Document all design decisions
- Keep FIPS 140-3 IG (Implementation Guidance) handy for reference

---

## Next Steps

**Immediate Action**: Start with Phase 1, Task 1.1 - Generate ML-KEM golden values

**Command to begin**:
```bash
# Create a test program to generate golden values
cargo test --features "ml-kem,ml-dsa,fips_140_3" -- --nocapture
```
