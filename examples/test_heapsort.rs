use sort_playground::{heapsort::heap_sort, perform_test};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST HEAP SORT ALGORITHM.");
    perform_test(RANDOM_PATH, "Heap sort. Random numbers.", heap_sort);
    perform_test(DIGITS_PATH, "Heap sort. Digits.", heap_sort);
    perform_test(SORTED_PATH, "Heap sort. Sorted array.", heap_sort);
    perform_test(REVERS_PATH, "Heap sort. Reversed array.", heap_sort);
    println!("{}", "=".repeat(80));
}
