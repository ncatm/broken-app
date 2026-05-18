use broken_app::concurrency;
use std::sync::{Mutex, OnceLock};

/// Глобальный счётчик общий для всех тестов — сериализуем доступ.
fn with_counter<F: FnOnce()>(f: F) {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _guard = LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    f();
}

#[test]
fn race_increment_is_correct() {
    with_counter(|| {
        let total = concurrency::race_increment(1_000, 4);
        assert_eq!(total, 4_000);
    });
}

#[test]
fn regression_concurrency_reset() {
    with_counter(|| {
        let _ = concurrency::race_increment(10, 2);
        concurrency::reset_counter();
        assert_eq!(concurrency::read_after_sleep(), 0);
    });
}
