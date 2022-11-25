use sort_playground::{demo, sort_playground::{SortPlayground, insertion, insertion_shift, insertion_binary}};

fn main() {
    println!("Simple insertion sort.");
    demo(
        &[10, 100, 1000, 10000, 100000], //, 1_000_000],
        SortPlayground::random,
        insertion,
    );
    println!("------------------------------------------");
    println!("Insertion sort with shift.");
    demo(
        &[10, 100, 1000, 10000, 100000], //, 1_000_000],
        SortPlayground::random,
        insertion_shift,
    );
    println!("------------------------------------------");
    println!("Insertion sort with binary search.");
    demo(
        &[10, 100, 1000, 10000, 100000], //1_000_000],
        SortPlayground::random,
        insertion_binary,
    )
}
