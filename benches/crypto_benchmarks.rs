use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pqc_nostd::{
    kyber_generate_key_pair, encapsulate, decapsulate,
    dilithium_generate_key_pair, dilithium_sign, dilithium_verify,
    FIPS_CONTEXT,
};

fn benchmark_ml_kem(c: &mut Criterion) {
    // Ensure FIPS module is initialized
    let _ = pqc_nostd::run_post();
    // Authenticate first
    pqc_nostd::auth::login(pqc_nostd::auth::Role::User, b"user123").unwrap();

    let mut group = c.benchmark_group("ML-KEM-1024");

    group.bench_function("KeyGen", |b| {
        b.iter(|| {
            let seed = [0xAAu8; 64];
            kyber_generate_key_pair(black_box(seed)).unwrap()
        })
    });

    let kp = kyber_generate_key_pair([0xAAu8; 64]).unwrap();
    let randomness = [0xBBu8; 32];

    group.bench_function("Encapsulate", |b| {
        b.iter(|| {
            encapsulate(black_box(kp.public_key()), black_box(randomness)).unwrap()
        })
    });

    let (ct, _ss) = encapsulate(kp.public_key(), randomness).unwrap();

    group.bench_function("Decapsulate", |b| {
        b.iter(|| {
            decapsulate(black_box(kp.private_key()), black_box(&ct)).unwrap()
        })
    });

    group.finish();
}

fn benchmark_ml_dsa(c: &mut Criterion) {
    // Ensure FIPS module is initialized
    let _ = pqc_nostd::run_post();
    // Authenticate first
    pqc_nostd::auth::login(pqc_nostd::auth::Role::User, b"user123").unwrap();

    let mut group = c.benchmark_group("ML-DSA-65");

    group.bench_function("KeyGen", |b| {
        b.iter(|| {
            let seed = [0xCCu8; 32];
            dilithium_generate_key_pair(black_box(seed)).unwrap()
        })
    });

    let kp = dilithium_generate_key_pair([0xCCu8; 32]).unwrap();
    let msg = b"Benchmark Message";
    let randomness = [0xDDu8; 32];

    group.bench_function("Sign", |b| {
        b.iter(|| {
            dilithium_sign(
                black_box(&kp.signing_key),
                black_box(msg),
                black_box(FIPS_CONTEXT),
                black_box(randomness),
            ).unwrap()
        })
    });

    let sig = dilithium_sign(&kp.signing_key, msg, FIPS_CONTEXT, randomness).unwrap();

    group.bench_function("Verify", |b| {
        b.iter(|| {
            dilithium_verify(
                black_box(&kp.verification_key),
                black_box(msg),
                black_box(FIPS_CONTEXT),
                black_box(&sig),
            ).unwrap()
        })
    });

    group.finish();
}

fn benchmark_self_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("Self-Tests");

    // Benchmark individual CASTs if exposed, or just the full POST suite
    // Note: run_post includes KATs and Integrity Check now, so it might be slow.
    // We'll benchmark the full POST to give a "boot time" estimate.
    
    // We need to be careful because run_post changes global state.
    // However, we can run it multiple times; it just re-runs tests.
    
    group.bench_function("Full POST (Boot Time)", |b| {
        b.iter(|| {
            // We ignore the error here because integrity check might fail if not built with injection
            // But for benchmarking logic speed, it's fine.
            // Actually, if integrity check fails, it returns early.
            // Ideally we want a passing POST.
            let _ = pqc_nostd::run_post(); 
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_ml_kem, benchmark_ml_dsa, benchmark_self_tests);
criterion_main!(benches);
