use sort_playground::{bubble, demo, sort_playground::SortPlayground};

fn main() {
    println!("Sort with naive bubble sort.");
    demo(
        &[10, 100, 1000, 10000, 100000], // 1_000_000],
        SortPlayground::random,
        bubble::bubble_sort,
    );
    println!("----------------------------------------");
    println!("Sort with optimized bubble sort.");
    demo(
        &[10, 100, 1000, 10000, 100000], // 1_000_000],
        SortPlayground::random,
        bubble::bubble_opt,
    );
}
