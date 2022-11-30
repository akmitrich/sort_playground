use sort_playground::{perform_test_lim, selection::selection_sort};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST SELECTION SORT ALGORITHM.");
    perform_test_lim(
        RANDOM_PATH,
        6,
        "Selection sort. Random numbers.",
        selection_sort,
    );
    perform_test_lim(DIGITS_PATH, 6, "Selection sort. Digits.", selection_sort);
    perform_test_lim(
        SORTED_PATH,
        6,
        "Selection sort. Sorted array.",
        selection_sort,
    );
    perform_test_lim(
        REVERS_PATH,
        6,
        "Selection sort. Reversed array.",
        selection_sort,
    );
    println!("{}", "=".repeat(80));
}
