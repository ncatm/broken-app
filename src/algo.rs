use std::collections::BTreeSet;

/// Уникальные значения в отсортированном порядке за O(n log n).
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    values
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Числа Фибоначчи за O(n) без лишних аллокаций.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0_u64;
            let mut b = 1_u64;
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
