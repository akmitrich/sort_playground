fn main() {
    let mut a = sort_playground::sort_playground::SortPlayground::random(5);
    println!(
        "{:?} {}%",
        a,
        a.sorted_percent()
    );
    a.insertion_binary();
    // let a = sort_playground::sort_playground::SortPlayground::reversed(5);
    println!(
        "{:?} {}%",
        a,
        a.sorted_percent(),
    );
}
