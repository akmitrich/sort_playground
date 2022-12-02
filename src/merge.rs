use crate::sort_playground::SortPlayground;

pub fn merge(left: &[i64], right: &[i64]) -> Vec<i64> {
    let left_size = left.len();
    let right_size = right.len();
    let mut a = Vec::with_capacity(left_size + right_size);
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left_size && right_index < right_size {
        if left[left_index] < right[right_index] {
            a.push(left[left_index]);
            left_index += 1;
        } else {
            a.push(right[right_index]);
            right_index += 1;
        }
    }
    if left_index < left_size {
        a.extend_from_slice(&left[left_index..]);
    }
    if right_index < right_size {
        a.extend_from_slice(&right[right_index..]);
    }
    a
}

pub fn merge_sort(mut merge: SortPlayground) -> SortPlayground {
    rec_merge(&mut merge.data);
    merge
}

fn rec_merge(a: &mut [i64]) {
    match a.len() {
        0..=1 => (), // do nothing
        size => {
            let mid = size / 2;
            rec_merge(&mut a[..mid]);
            rec_merge(&mut a[mid..]);
            a.copy_from_slice(merge(&a[..mid], &a[mid..]).as_slice());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = [0, 1, 42, 55, 67, 122];
        let b = [1, 3, 3];
        let merged = merge(&a, &b);
        assert_eq!(&[0, 1, 1, 3, 3, 42, 55, 67, 122], merged.as_slice());
    }

    #[test]
    fn test_subarrays() {
        let a = [0, 1, 3, 67, 1, 3, 42, 55, 122];
        let merged = merge(&a[..4], &a[4..]);
        assert_eq!(&[0, 1, 1, 3, 3, 42, 55, 67, 122], merged.as_slice());
    }
}
