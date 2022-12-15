use crate::sort_playground::SortPlayground;

type Counting = Vec<usize>;

pub fn counting_sort(mut counting: SortPlayground) -> SortPlayground {
    counting.cmp += 2 * counting.data.len();
    let (min, max) = get_min_max(counting.data.iter().copied());
    counting.asg += counting.data.len();
    let count = calculate_count(counting.data.iter().copied(), min, max);
    counting.asg += count.len() - 1;
    let prefix_sum = calculate_prefix_sum(count);
    counting.asg += 2 * counting.data.len();
    counting.data = make_sorted(&counting.data, prefix_sum, min);
    counting
}

fn make_sorted(data: &[i64], mut prefix: Vec<usize>, min: i64) -> Vec<i64> {
    let mut output = vec![0; data.len()];
    for n in data.iter().rev() {
        let value = *n;
        let j = index(value, min);
        prefix[j] -= 1;
        output[prefix[j]] = value;
    }
    output
}

fn calculate_prefix_sum(mut count: Vec<usize>) -> Vec<usize> {
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }
    count
}

fn get_min_max(data: impl Iterator<Item = i64>) -> (i64, i64) {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for n in data {
        if n < min {
            min = n
        }
        if n > max {
            max = n
        }
    }
    (min, max)
}

fn calculate_count(data: impl Iterator<Item = i64>, min: i64, max: i64) -> Counting {
    let size = max - min + 1;
    assert!(size > 0);
    let size = size as usize;
    let mut count = vec![0; size];
    for n in data {
        count[index(n, min)] += 1;
    }
    count
}

fn index(value: i64, min: i64) -> usize {
    let index = value - min;
    assert!(index >= 0);
    index as _
}

#[cfg(test)]
mod tests {
    use crate::sort_playground::sort;

    use super::*;

    #[test]
    fn test_count() {
        let c = calculate_count([1, 2, 4, 4, 2, 2, 2, 1, 4, 5].into_iter(), 1, 5);
        assert_eq!(3, c[3]);
        assert_eq!(2, c[0]);
        assert_eq!(1, c[4]);
        let mut i = c.iter();
        let one = i.next().unwrap();
        assert_eq!(2, *one);
        let two = i.next().unwrap();
        assert_eq!(4, *two);
    }

    #[test]
    fn test_sorting() {
        let playground = sort((-2..=2).rev(), counting_sort);
        assert_eq!(vec![-2, -1, 0, 1, 2], playground.data);
    }
}
