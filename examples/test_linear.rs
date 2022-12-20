use sort_playground::{
    linear::{counting_sort, generic_bucket_sort, radix_sort_4bits, radix_sort_8bits},
    perform_test, perform_test_lim,
};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST BUCKET SORT ALGORITHM.");
    perform_test(
        RANDOM_PATH,
        "Bucket sort. Random numbers.",
        generic_bucket_sort,
    );
    perform_test_lim(DIGITS_PATH, 6, "Bucket sort. Digits.", generic_bucket_sort);
    perform_test(
        SORTED_PATH,
        "Bucket sort. Sorted array.",
        generic_bucket_sort,
    );
    perform_test(
        REVERS_PATH,
        "Bucket sort. Reversed array.",
        generic_bucket_sort,
    );
    println!("{}", "=".repeat(80));
    println!("{}", "=".repeat(80));
    println!("TEST COUNTING SORT ALGORITHM.");
    perform_test(RANDOM_PATH, "Counting sort. Random numbers.", counting_sort);
    perform_test(DIGITS_PATH, "Counting sort. Digits.", counting_sort);
    perform_test(SORTED_PATH, "Counting sort. Sorted array.", counting_sort);
    perform_test(REVERS_PATH, "Counting sort. Reversed array.", counting_sort);
    println!("{}", "=".repeat(80));
    println!("{}", "=".repeat(80));
    println!("TEST RADIX SORT ALGORITHM (8 bits).");
    perform_test(RANDOM_PATH, "Radix sort. Random numbers.", radix_sort_8bits);
    perform_test(DIGITS_PATH, "Radix sort. Digits.", radix_sort_8bits);
    perform_test(SORTED_PATH, "Radix sort. Sorted array.", radix_sort_8bits);
    perform_test(REVERS_PATH, "Radix sort. Reversed array.", radix_sort_8bits);
    println!("{}", "=".repeat(80));
    println!("TEST RADIX SORT ALGORITHM (4 bits).");
    perform_test(RANDOM_PATH, "Radix sort. Random numbers.", radix_sort_4bits);
    perform_test(DIGITS_PATH, "Radix sort. Digits.", radix_sort_4bits);
    perform_test(SORTED_PATH, "Radix sort. Sorted array.", radix_sort_4bits);
    perform_test(REVERS_PATH, "Radix sort. Reversed array.", radix_sort_4bits);
    println!("{}", "=".repeat(80));
}
