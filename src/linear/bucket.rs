use crate::sort_playground::SortPlayground;

pub fn generic_bucket_sort(mut bucket: SortPlayground) -> SortPlayground {
    let number_of_buckets = bucket.data.len();
    let mut buckets: Vec<Vec<i64>> = vec![vec![]; number_of_buckets];
    let number_of_buckets = number_of_buckets as i64;
    let max_plus_1 = 1 + *bucket.data.iter().max().unwrap();
    for value in bucket.data.iter() {
        let bucket_index = *value * number_of_buckets / max_plus_1;
        let bucket_to_add = buckets.get_mut(bucket_index as usize).unwrap();
        let index = match bucket_to_add.binary_search(value) {
            Ok(i) => i,
            Err(i) => i,
        };
        bucket_to_add.insert(index, *value);
    }
    let mut new_data = vec![];
    for b in buckets {
        new_data.extend(b.into_iter());
    }
    bucket.data = new_data;
    bucket
}
