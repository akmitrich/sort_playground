use sort_playground::sort_playground::{shell_naive, sort, sorted_percent, SortPlayground};

fn main() {
    let a = SortPlayground::random(5);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
    let a = sort(a.into_iter(), shell_naive);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
}
