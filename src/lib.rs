use std::time::{Duration, Instant};

pub mod sort_playground;

pub fn run_elapsed<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}