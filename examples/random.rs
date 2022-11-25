use sort_playground::{sorted_percent, shell::shell_naive, sort_playground::{sort, SortPlayground}};

fn main() {
    let a = SortPlayground::random(5);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
    let a = sort(a.into_iter(), shell_naive);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
}
