use std::time::{Duration, Instant};

use sort_playground::SortPlayground;

pub mod sort_playground;

pub fn run_elapsed<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

pub fn demo<Init, Sort>(n: &[usize], init: Init, sort: Sort)
where
    Init: Fn(usize) -> SortPlayground,
    Sort: Fn(&mut SortPlayground),
{
    for n in n {
        let mut a = init(*n);
        println!("Going to sort {}% sorted array of {n} numbers", a.sorted_percent());
        let elapsed = run_elapsed(|| {
            sort(&mut a);
        });
        println!("Sorted {n} numbers in {elapsed:?}. {}", a.get_report());
    }
}
