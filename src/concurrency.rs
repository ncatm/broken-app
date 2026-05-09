use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Инкремент общего счётчика из нескольких потоков с атомарной синхронизацией.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::SeqCst);
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..iterations {
                    COUNTER.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("thread panicked");
    }
    COUNTER.load(Ordering::SeqCst)
}

pub fn read_after_sleep() -> u64 {
    thread::sleep(Duration::from_millis(10));
    COUNTER.load(Ordering::SeqCst)
}

pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}
