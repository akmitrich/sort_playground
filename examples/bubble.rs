use sort_playground::{demo, sort_playground::SortPlayground};

fn main() {
    demo(
        &[10, 100, 1000, 10000, 100000, 1_000_000],
        SortPlayground::random,
        SortPlayground::bubble_sort,
    );
}
