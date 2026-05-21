use broken_app::{algo, sum_even};
use pprof::ProfilerGuardBuilder;
use std::fs::{File, create_dir_all};
use std::hint::black_box;

fn workload() {
    let dedup_data: Vec<u64> = (0..20_000).flat_map(|n| [n, n]).collect();
    let sum_data: Vec<i64> = (0..200_000).collect();

    for _ in 0..600 {
        black_box(algo::slow_dedup(&dedup_data));
        black_box(sum_even(&sum_data));
        for n in 34..=38 {
            black_box(algo::slow_fib(n));
        }
    }
}

fn main() {
    create_dir_all("artifacts").expect("failed to create artifacts dir");

    let guard = ProfilerGuardBuilder::default()
        .frequency(999)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .expect("failed to start profiler");

    workload();

    let report = guard.report().build().expect("failed to build report");
    let file = File::create("artifacts/flamegraph.svg").expect("failed to create flamegraph");
    report.flamegraph(file).expect("failed to write flamegraph");

    println!("flamegraph saved to artifacts/flamegraph.svg");
}
