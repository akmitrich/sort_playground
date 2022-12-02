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
    let mut asg = 0;
    let mut cmp = 0;
    rec_merge(&mut merge.data, &mut asg, &mut cmp);
    merge.asg = asg;
    merge.cmp = cmp;
    merge
}

fn rec_merge(a: &mut [i64], asg: &mut usize, cmp: &mut usize) {
    match a.len() {
        0..=1 => (), // do nothing
        size => {
            let mid = size / 2;
            rec_merge(&mut a[..mid], asg, cmp);
            rec_merge(&mut a[mid..], asg, cmp);
            *asg += size;
            a.copy_from_slice(merge_playground(&a[..mid], &a[mid..], asg, cmp).as_slice());
        }
    }
}

fn merge_playground(left: &[i64], right: &[i64], asg: &mut usize, cmp: &mut usize) -> Vec<i64> {
    let left_size = left.len();
    let right_size = right.len();
    let mut a = Vec::with_capacity(left_size + right_size);
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left_size && right_index < right_size {
        *cmp += 1;
        if left[left_index] < right[right_index] {
            *asg += 1;
            a.push(left[left_index]);
            left_index += 1;
        } else {
            *asg += 1;
            a.push(right[right_index]);
            right_index += 1;
        }
    }
    if left_index < left_size {
        *asg += left_size - left_index - 1;
        a.extend_from_slice(&left[left_index..]);
    }
    if right_index < right_size {
        *asg += right_size - right_index - 1;
        a.extend_from_slice(&right[right_index..]);
    }
    a
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
