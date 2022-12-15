use std::time::Instant;

use sort_playground::SortPlayground;

pub mod bubble;
pub mod external;
pub mod heapsort;
pub mod insertion;
pub mod linear;
pub mod merge;
pub mod quick;
pub mod selection;
pub mod shell;
pub mod sort_playground;
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

pub fn perform_test<Sort: Fn(SortPlayground) -> SortPlayground>(
    path: &str,
    title: &str,
    method: Sort,
) {
    let hline: String = "-".repeat(80);
    println!("{hline}");
    println!("{title}");
    tester::run_silently(path, |data| {
        let playground =
            sort_playground::sort(data[1].split(' ').map(|x| x.parse().unwrap()), &method);
        println!("{}", playground.get_report());
        playground
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    });
    println!("{hline}");
}

pub fn perform_test_lim<Sort: Fn(SortPlayground) -> SortPlayground>(
    path: &str,
    max_test: u8,
    title: &str,
    method: Sort,
) {
    let hline: String = "-".repeat(80);
    println!("{hline}");
    println!("{title}");
    tester::run_test_lim(
        path,
        |data| {
            let playground =
                sort_playground::sort(data[1].split(' ').map(|x| x.parse().unwrap()), &method);
            println!("{}", playground.get_report());
            playground
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        },
        max_test,
    );
    println!("{hline}");
}

pub fn sorted_percent<'a>(mut data: impl Iterator<Item = &'a i64>) -> usize {
    let start = data.next().unwrap();
    let (sum, success, _) = data.fold((0, 0, start), |(sum, success, prev), current| {
        (
            sum + 1,
            if prev <= current {
                success + 1
            } else {
                success
            },
            current,
        )
    });
    if sum > 0 {
        success * 100 / sum
    } else {
        100
    }
}
