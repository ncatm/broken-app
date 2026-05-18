use std::time::Instant;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            out.push(*v);
            out.sort_unstable();
        }
    }
    out
}

fn main() {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    let start = Instant::now();
    let _ = slow_dedup(&data);
    println!("slow_dedup (broken): {:?}", start.elapsed());

    for n in [20u64, 25, 30, 32] {
        let start = Instant::now();
        let _ = slow_fib(n);
        println!("slow_fib({n}) (broken): {:?}", start.elapsed());
    }
}
