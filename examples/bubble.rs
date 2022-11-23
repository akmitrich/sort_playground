fn main() {
    let n = vec![10, 100, 1000, 10000, 100000];
    for n in n.iter() {
        let mut a = sort_playground::sort_playground::SortPlayground::random(*n);
        let elapsed = sort_playground::run_elapsed(|| {
            a.bubble_sort()
        });
        println!("Sorted {n} numbers in {elapsed:?}");
    }
}