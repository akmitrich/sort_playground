fn main() {
    let mut a = sort_playground::sort_playground::SortPlayground::random(5);
    println!("{:?} {}%", a, a.sorted_percent());
    a.shell_naive();
    // let a = sort_playground::sort_playground::SortPlayground::reversed(5);
    println!("{:?} {}%", a, a.sorted_percent(),);
}
