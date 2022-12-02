use sort_playground::{perform_test, merge::merge_sort};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST MERGE SORT ALGORITHM.");
    perform_test(RANDOM_PATH, "Quick sort. Random numbers.", merge_sort);
    perform_test(DIGITS_PATH,  "Quick sort. Digits.", merge_sort);
    perform_test(SORTED_PATH, "Quick sort. Sorted array.", merge_sort);
    perform_test(REVERS_PATH, "Quick sort. Reversed array.", merge_sort);
    println!("{}", "=".repeat(80));
}
