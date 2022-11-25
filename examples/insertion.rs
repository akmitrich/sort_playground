use sort_playground::{demo, insertion, sort_playground::{SortPlayground}};

fn main() {
    println!("Simple insertion sort.");
    demo(
        &[10, 100, 1000, 10000, 100000], //, 1_000_000],
        SortPlayground::random,
        insertion::insertion,
    );
    println!("------------------------------------------");
    println!("Insertion sort with shift.");
    demo(
        &[10, 100, 1000, 10000, 100000], //, 1_000_000],
        SortPlayground::random,
        insertion::insertion_shift,
    );
    println!("------------------------------------------");
    println!("Insertion sort with binary search.");
    demo(
        &[10, 100, 1000, 10000, 100000], //1_000_000],
        SortPlayground::random,
        insertion::insertion_binary,
    )
}
