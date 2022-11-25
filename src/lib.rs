use std::time::Instant;

use sort_playground::{SortPlayground};

pub mod sort_playground;
pub mod bubble;
pub mod insertion;
pub mod shell;
pub mod tester;

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

pub fn sorted_percent<'a>(mut data: impl Iterator<Item = &'a i64>) -> usize {
    let start = data.next().unwrap();
    let (sum, success, _) = data.fold((0, 0, start), |(sum, success, prev), current| {
        (
            sum + 1,
            if prev < current { success + 1 } else { success },
            current,
        )
    });
    success * 100 / sum
}

