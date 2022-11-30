use crate::sort_playground::SortPlayground;

pub fn selection_sort(mut sel: SortPlayground) -> SortPlayground {
    let size = sel.data.len();
    for i in 0..(size - 1) {
        sel.cmp += size - i - 1; //we have to do this many comparisons to find min index
        sel.swap(i, find_min_index(i, &sel.data));
    }
    sel
}

fn find_min_index(i: usize, data: &[i64]) -> usize {
    let mut min_index = i;
    for j in (i + 1)..data.len() {
        if data[j] < data[min_index] {
            min_index = j;
        }
    }
    min_index
}
