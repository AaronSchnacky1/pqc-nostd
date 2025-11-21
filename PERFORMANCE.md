# Performance Benchmarks

**Project**: pqc-nostd v0.1.0  
**Date**: 2025-11-20  
**Environment**: Windows x86_64 (Host)

## 📊 Summary

| Algorithm | Operation | Time (Mean) | Iterations/Sec |
|-----------|-----------|-------------|----------------|
| **ML-KEM-1024** | KeyGen | 33.7 µs | ~29,600 |
| | Encapsulate | 36.3 µs | ~27,500 |
| | Decapsulate | 42.4 µs | ~23,500 |
| **ML-DSA-65** | KeyGen | 83.3 µs | ~12,000 |
| | Sign | 195.6 µs | ~5,100 |
| | Verify | 84.0 µs | ~11,900 |
| **System** | Full POST (Boot) | 2.25 ms | ~440 |

## 🚀 Analysis

### ML-KEM-1024 (FIPS 203)
- **Key Generation**: Extremely fast (~34µs). Suitable for frequent rotation.
- **Encapsulation**: Very fast (~36µs). Negligible impact on handshake latency.
- **Decapsulation**: Fast (~42µs). Slightly slower than encapsulation but highly performant.

### ML-DSA-65 (FIPS 204)
- **Key Generation**: Fast (~83µs).
- **Signing**: Moderate (~196µs). This is the most expensive operation but still supports >5,000 signatures/second on a single core.
- **Verification**: Fast (~84µs). Efficient for verifying signatures (e.g., secure boot, firmware updates).

### System Boot Time
- **Full POST**: ~2.25 ms
- This includes:
    - SHA-3/SHAKE CASTs
    - ML-KEM KATs (KeyGen, Encaps, Decaps)
    - ML-DSA KATs (KeyGen, Sign, Verify)
    - Integrity Check (HMAC-SHA-256 of code segment)
    - PCTs (Pairwise Consistency Tests)
- **Conclusion**: The FIPS self-tests add negligible overhead to boot time (milliseconds).

## 📝 Methodology
- **Framework**: Criterion.rs v0.5
- **Optimization**: `release` profile (`opt-level = 3`, `lto = true` implied)
- **Samples**: 100 samples per benchmark
- **Warmup**: 3.0 seconds
- **Hardware**: Host CPU (Windows x86_64)

*Note: Performance on embedded targets (ARM Cortex-M, RISC-V) will be lower and depends on clock speed and memory architecture.*
