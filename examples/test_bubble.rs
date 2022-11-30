use sort_playground::{
    bubble::{bubble_opt, bubble_sort},
    perform_test_lim,
};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST BUBBLE SORT ALGORITHMS.");
    perform_test_lim(
        RANDOM_PATH,
        5,
        "Bubble sort classic. Random numbers.",
        bubble_sort,
    );
    perform_test_lim(
        RANDOM_PATH,
        5,
        "Bubble sort optimized. Random numbers.",
        bubble_opt,
    );
    perform_test_lim(DIGITS_PATH, 6, "Bubble sort classic. Digits.", bubble_sort);
    perform_test_lim(DIGITS_PATH, 6, "Bubble sort optimized. Digits.", bubble_opt);
    perform_test_lim(
        SORTED_PATH,
        5,
        "Bubble sort classic. Sorted array.",
        bubble_sort,
    );
    perform_test_lim(
        SORTED_PATH,
        5,
        "Bubble sort optimized. Sorted array.",
        bubble_opt,
    );
    perform_test_lim(
        REVERS_PATH,
        5,
        "Bubble sort classic. Reversed array.",
        bubble_sort,
    );
    perform_test_lim(
        REVERS_PATH,
        5,
        "Bubble sort optimized. Reversed array.",
        bubble_opt,
    );
    println!("{}", "=".repeat(80));
}
