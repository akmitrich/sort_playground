use sort_playground::{demo, sort_playground::{SortPlayground, shell_naive, shell_ciura, shell1}};

fn main() {
    println!("Sort with naive shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell_naive,
    );
    println!("----------------------------------------");
    println!("Ciura gaps shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell_ciura,
    );
    println!("----------------------------------------");
    println!("Some gaps shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell1,
    );
}
