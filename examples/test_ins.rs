use sort_playground::{
    insertion::{insertion, insertion_binary, insertion_shift},
    perform_test_lim,
};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST INSERTION SORT ALGORITHMS.");
    perform_test_lim(
        RANDOM_PATH,
        5,
        "Insertion sort classic. Random numbers.",
        insertion,
    );
    perform_test_lim(
        RANDOM_PATH,
        5,
        "Insertion sort with shift. Random numbers.",
        insertion_shift,
    );
    perform_test_lim(
        RANDOM_PATH,
        5,
        "Insertion sort with shift and binary search. Random numbers.",
        insertion_binary,
    );
    perform_test_lim(DIGITS_PATH, 5, "Insertion sort classic. Digits.", insertion);
    perform_test_lim(
        DIGITS_PATH,
        5,
        "Insertion sort with shift. Digits.",
        insertion_shift,
    );
    perform_test_lim(
        DIGITS_PATH,
        5,
        "Insertion sort with shift and binary search. Digits.",
        insertion_binary,
    );
    perform_test_lim(
        SORTED_PATH,
        5,
        "Insertion sort classic. Sorted array.",
        insertion,
    );
    perform_test_lim(
        SORTED_PATH,
        5,
        "Insertion sort with shift. Sorted array.",
        insertion_shift,
    );
    perform_test_lim(
        SORTED_PATH,
        5,
        "Insertion sort with shift and binary search. Sorted array.",
        insertion_binary,
    );
    perform_test_lim(
        REVERS_PATH,
        5,
        "Insertion sort classic. Reversed array.",
        insertion,
    );
    perform_test_lim(
        REVERS_PATH,
        5,
        "Insertion sort with shift. Reversed array.",
        insertion_shift,
    );
    perform_test_lim(
        REVERS_PATH,
        5,
        "Insertion sort with shift and binary search. Reversed array.",
        insertion_binary,
    );
    println!("{}", "=".repeat(80));
}
