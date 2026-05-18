use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]);
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

// --- Регрессионные тесты на исправленные дефекты ---

#[test]
fn regression_sum_even_no_out_of_bounds() {
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[2, 4, 6]), 12);
}

#[test]
fn regression_leak_buffer_empty_input() {
    assert_eq!(leak_buffer(&[]), 0);
}

#[test]
fn regression_normalize_collapses_internal_whitespace() {
    assert_eq!(normalize("  foo\t\nbar  "), "foobar");
}

#[test]
fn regression_average_positive_ignores_non_positive() {
    assert_eq!(broken_app::average_positive(&[-1, -2]), 0.0);
    assert!((broken_app::average_positive(&[10, 20]) - 15.0).abs() < f64::EPSILON);
}

#[test]
fn regression_dedup_large_input() {
    let input: Vec<u64> = (0..1_000).flat_map(|n| [n, n]).collect();
    let uniq = algo::slow_dedup(&input);
    assert_eq!(uniq.len(), 1_000);
    assert_eq!(uniq[0], 0);
    assert_eq!(uniq[999], 999);
}

#[test]
fn regression_fib_zero_and_one() {
    assert_eq!(algo::slow_fib(0), 0);
    assert_eq!(algo::slow_fib(1), 1);
}
