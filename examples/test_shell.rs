use sort_playground::{
    perform_test,
    shell::{shell1, shell_ciura, shell_naive},
};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    println!("{}", "=".repeat(80));
    println!("TEST SHELL SORT ALGORITHMS.");
    perform_test(
        RANDOM_PATH,
        "Shell sort with gap /= 2. Random numbers.",
        shell_naive,
    );
    perform_test(
        RANDOM_PATH,
        "Shell sort with kinda Ciura gaps. Random numbers.",
        shell_ciura,
    );
    perform_test(
        RANDOM_PATH,
        "Shell sort with some great gaps. Random numbers.",
        shell1,
    );
    perform_test(
        DIGITS_PATH,
        "Shell sort with gap /= 2. Digits.",
        shell_naive,
    );
    perform_test(
        DIGITS_PATH,
        "Shell sort with kinda Ciura gaps. Digits.",
        shell_ciura,
    );
    perform_test(
        DIGITS_PATH,
        "Shell sort with some great gaps. Digits.",
        shell1,
    );
    perform_test(
        SORTED_PATH,
        "Shell sort with gap /= 2. Sorted array.",
        shell_naive,
    );
    perform_test(
        SORTED_PATH,
        "Shell sort with kinda Ciura gaps. Sorted array.",
        shell_ciura,
    );
    perform_test(
        SORTED_PATH,
        "Shell sort with some great gaps. Sorted array.",
        shell1,
    );
    perform_test(
        REVERS_PATH,
        "Shell sort with gap /= 2. Reversed array.",
        shell_naive,
    );
    perform_test(
        REVERS_PATH,
        "Shell sort with kinda Ciura gaps. Reversed array.",
        shell_ciura,
    );
    perform_test(
        REVERS_PATH,
        "Shell sort with some great gaps. Reversed array.",
        shell1,
    );
    println!("{}", "=".repeat(80));
}
