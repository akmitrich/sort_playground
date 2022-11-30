use crate::sort_playground::SortPlayground;

pub fn insertion(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        for i in (0..j).rev() {
            ins.cmp += 1;
            if ins.data[i] <= ins.data[i + 1] {
                break;
            }
            ins.swap(i, i + 1);
        }
    }
    ins
}

pub fn insertion_shift(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        let item = ins.data[j];
        let mut found_index = 0;
        for i in (0..j).rev() {
            ins.cmp += 1;
            if ins.data[i] > item {
                ins.asg += 1;
                ins.data[i + 1] = ins.data[i];
            } else {
                found_index = i + 1;
                break;
            }
        }
        ins.asg += 1;
        ins.data[found_index] = item;
    }
    ins
}

pub fn insertion_binary(mut ins: SortPlayground) -> SortPlayground {
    for j in 1..ins.data.len() {
        let x = ins.data[j];
        let found_index = insort_left(&mut ins, x, 0, j);
        perform_insertion(&mut ins, j, found_index);
    }
    ins
}

fn insort_left(ins: &mut SortPlayground, value: i64, a: usize, b: usize) -> usize {
    let mut low = a;
    let mut high = b;
    loop {
        ins.cmp += 1;
        if low < high {
            let mid = low + (high - low) / 2;
            if ins.data[mid] < value {
                low = mid + 1;
            } else {
                high = mid;
            }
        } else {
            break low;
        }
    }
}

fn perform_insertion(ins: &mut SortPlayground, from: usize, to: usize) {
    assert!(from < ins.data.len());
    assert!(to <= from);
    let x = ins.data[from];
    for i in (to..from).rev() {
        ins.asg += 1;
        ins.data[i + 1] = ins.data[i];
    }
    ins.asg += 1;
    ins.data[to] = x;
}
