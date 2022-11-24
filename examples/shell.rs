use sort_playground::{demo, sort_playground::SortPlayground};

fn main() {
    println!("Sort with naive shell sort.");
    demo(
        &[10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000, 100_000_000],
        SortPlayground::random,
        SortPlayground::shell_naive,
    );
    println!("----------------------------------------");
}