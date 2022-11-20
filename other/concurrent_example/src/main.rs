use concurrent_example::is_prime;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn main() {
    const MAX: u32 = 200_000;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_1 = counter.clone();
    let counter_2 = counter.clone();

    let t1 = std::thread::spawn(move || {
        counter_1.fetch_add(
            (2..MAX / 2).filter(|i| is_prime(*i)).count(),
            Ordering::SeqCst,
        )
    });
    let t2 = std::thread::spawn(move || {
        counter_2.fetch_add(
            (MAX / 2..MAX).filter(|i| is_prime(*i)).count(),
            Ordering::SeqCst,
        )
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!(
        "Found {} prime numbers in the range 2..{MAX}",
        counter.load(Ordering::SeqCst)
    );
}
