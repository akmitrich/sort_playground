use crate::sort_playground::SortPlayground;


pub fn bubble_sort(mut bubble: SortPlayground) -> SortPlayground {
    for j in (0..bubble.data.len()).rev() {
        for i in 0..j {
            bubble.cmp += 1;
            if bubble.data[i] > bubble.data[i + 1] {
                bubble.swap(i, i + 1);
            }
        }
    }
    bubble
}

pub fn bubble_opt(mut bubble: SortPlayground) -> SortPlayground {
    for j in (0..bubble.data.len()).rev() {
        let mut may_stop = true;
        for i in 0..j {
            bubble.cmp += 1;
            if bubble.data[i] > bubble.data[i + 1] {
                may_stop = false;
                bubble.swap(i, i + 1);
            }
        }
        if may_stop {
            break;
        }
    }
    bubble
}
