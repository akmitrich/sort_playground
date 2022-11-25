use sort_playground::{demo, shell, sort_playground::{SortPlayground}};

fn main() {
    println!("Sort with naive shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell::shell_naive,
    );
    println!("----------------------------------------");
    println!("Ciura gaps shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell::shell_ciura,
    );
    println!("----------------------------------------");
    println!("Some gaps shell sort.");
    demo(
        &[
            10, 100, 1000, 10000, 100000, 1_000_000, 10_000_000,
            //            100_000_000,
        ],
        SortPlayground::random,
        shell::shell1,
    );
}
