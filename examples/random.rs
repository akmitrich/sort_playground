use sort_playground::{
    shell::shell_naive,
    sort_playground::{sort, SortPlayground},
    sorted_percent,
};

fn main() {
    let a = SortPlayground::random(5);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
    let a = sort(a.into_iter(), shell_naive);
    println!("{:?} {}%", a, sorted_percent(a.iter()),);
    let a = sort_playground::merge::merge(&[0, 1, 42, 55, 67, -22], &[1, 3, 3]);
    println!("{a:?}");
}
