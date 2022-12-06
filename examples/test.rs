use sort_playground::tester::{run_test_with_path, run_test_lim};
use std::path::Path;

fn main() {
    let path_random = Path::new("..").join("sorting-tests").join("0.random");
    run_test_lim(
        path_random,
        |a| String::from(*a.first().unwrap()),
        5,
    );
}
