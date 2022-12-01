use sort_playground::{quick::quick_sort, perform_test, perform_test_lim};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST QUICK SORT ALGORITHM.");
    perform_test(RANDOM_PATH, "Quick sort. Random numbers.", quick_sort);
    perform_test_lim(DIGITS_PATH, 6, "Quick sort. Digits.", quick_sort);
    perform_test(SORTED_PATH, "Quick sort. Sorted array.", quick_sort);
    perform_test(REVERS_PATH, "Quick sort. Reversed array.", quick_sort);
    println!("{}", "=".repeat(80));
}
