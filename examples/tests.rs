use sort_playground::sort_playground::{shell1, shell_ciura, shell_naive, sort, SortPlayground};

const RANDOM_PATH: &str = "../sorting-tests/0.random";
const DIGITS_PATH: &str = "../sorting-tests/1.digits";
const SORTED_PATH: &str = "../sorting-tests/2.sorted";
const REVERS_PATH: &str = "../sorting-tests/3.revers";

fn main() {
    test_shell();
}

fn test_shell() {
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

fn perform_test<Sort: Fn(SortPlayground) -> SortPlayground>(path: &str, title: &str, method: Sort) {
    let hline: String = "-".repeat(80);
    println!("{hline}");
    println!("{title}");
    sort_playground::tester::run_test(path, |data| {
        sort(data[1].split(' ').map(|x| x.parse().unwrap()), &method)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    });
    println!("{hline}");
}
