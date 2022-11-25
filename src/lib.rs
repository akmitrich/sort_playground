use std::time::{Duration, Instant};

use sort_playground::{SortPlayground, sorted_percent};

pub mod sort_playground;
pub mod tester;

pub fn run_elapsed<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

pub fn demo<Init, Sort>(n: &[usize], init: Init, sort: Sort)
where
    Init: Fn(usize) -> SortPlayground,
    Sort: Fn(SortPlayground) -> SortPlayground,
{
    for n in n {
        let a = init(*n);
        println!(
            "Going to sort {}% sorted array of {n} numbers",
            sorted_percent(a.iter())
        );
        let start = Instant::now();
        let a = sort(a);
        let elapsed = Instant::now().duration_since(start);
        println!("Sorted {n} numbers in {elapsed:?}. {}", a.get_report());
    }
}
