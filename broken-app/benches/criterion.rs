use broken_app::{algo, sum_even};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).collect();
    c.bench_function("sum_even", |b| b.iter(|| sum_even(&data)));
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("slow_fib", |b| b.iter(|| algo::slow_fib(32)));
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("slow_dedup", |b| {
        b.iter_batched(
            || data.clone(),
            |v| {
                let _ = algo::slow_dedup(&v);
            },
            BatchSize::SmallInput,
        )
    });
}

#[cfg(unix)]
fn criterion_config() -> Criterion {
    Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
}

#[cfg(not(unix))]
fn criterion_config() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_sum_even, bench_fib, bench_dedup
}
criterion_main!(benches);
