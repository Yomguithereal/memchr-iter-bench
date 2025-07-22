use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use memchr_iter_bench::{
    memchr_sse2_iter, memchr_sse2_loop, memchr_sse2_loop_amortized, scalar_baseline,
};

fn group(c: &mut Criterion, name: &str, haystack: &[u8]) {
    let mut group = c.benchmark_group(name);

    let haystack = black_box(haystack);

    group.bench_function("scalar_baseline", |b| {
        b.iter(|| scalar_baseline(b',', haystack));
    });
    group.bench_function("memchr_sse2_loop", |b| {
        b.iter(|| memchr_sse2_loop(b',', haystack));
    });
    group.bench_function("memchr_sse2_loop_amortized", |b| {
        b.iter(|| memchr_sse2_loop_amortized(b',', haystack));
    });
    group.bench_function("memchr_sse2_iter", |b| {
        b.iter(|| memchr_sse2_iter(b',', haystack));
    });

    group.finish();
}

fn bench(c: &mut Criterion) {
    group(c, "empty_string", b"");
    group(c, "very_short_string", b"a,c");
    group(c, "very_short_string_no_matches", b"abc");

    group(c, "short_string_close_matches", b"name,surname,age,color");
    group(c, "short_string_no_matches", b"thereisnothingheretobefound");

    group(
        c,
        "long_string_no_matches",
        &b"thereisnothingheretobefound".repeat(5000),
    );
    group(
        c,
        "long_string_close_matches",
        &b"name,surname,age,color".repeat(5000),
    );
    group(
        c,
        "long_string_sparse_matches",
        &b"thereisnothing,heretobefound".repeat(5000),
    );
    group(
        c,
        "long_string_all_matches",
        &b",,,,,,,,,,,,,,,,,,,,,,,,,,,,".repeat(5000),
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
