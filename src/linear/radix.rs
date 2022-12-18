use crate::sort_playground::SortPlayground;

pub fn radix_sort_8bits(mut radix: SortPlayground) -> SortPlayground {
    for byte_index in 0..8 {
        radix.asg += radix.data.len();
        let mut count = make_count(&radix.data, 8, byte_index);
        radix.asg += count.len();
        make_prefix_sum(&mut count);
        radix.asg += radix.data.len() * 2;
        radix.data = make_next_step(&radix.data, &mut count, 8, byte_index);
    }
    radix
}

pub fn radix_sort_4bits(mut radix: SortPlayground) -> SortPlayground {
    for radix_index in 0..16 {
        radix.asg += radix.data.len();
        let mut count = make_count(&radix.data, 4, radix_index);
        radix.asg += count.len();
        make_prefix_sum(&mut count);
        radix.asg += radix.data.len() * 2;
        radix.data = make_next_step(&radix.data, &mut count, 4, radix_index);
    }
    radix
}

fn make_count(data: &[i64], radix_size: usize, radix_index: usize) -> Vec<usize> {
    assert!(radix_size > 0);
    let radix_mask = radix_mask(radix_size);
    let mut count = vec![0; 1 << radix_size];
    for value in data.iter() {
        let radix_piece = radix_piece(value, radix_size, radix_index, radix_mask);
        count[radix_piece] += 1;
    }
    count
}

fn make_prefix_sum(count: &mut [usize]) {
    let mut total = 0;
    for count in count.iter_mut() {
        let offset = *count;
        *count = total;
        total += offset;
    }
}

fn make_next_step(
    data: &[i64],
    prefix_sum: &mut [usize],
    radix_size: usize,
    radix_index: usize,
) -> Vec<i64> {
    let mut buffer = vec![0; data.len()];
    let radix_mask = radix_mask(radix_size);
    for value in data.iter() {
        let radix_piece = radix_piece(value, radix_size, radix_index, radix_mask);
        buffer[prefix_sum[radix_piece]] = *value;
        prefix_sum[radix_piece] += 1;
    }
    buffer
}

fn radix_mask(radix_size: usize) -> u64 {
    let mut radix_mask = 0;
    for bit in 0..radix_size {
        radix_mask |= 1 << bit;
    }
    radix_mask
}

fn radix_piece(value: &i64, radix_size: usize, radix_index: usize, radix_mask: u64) -> usize {
    let radix_value = *value as u64;
    ((radix_value >> (radix_size * radix_index)) & radix_mask) as _
}

#[cfg(test)]
mod tests {
    use crate::sort_playground::sort;

    use super::*;

    #[test]
    fn it_works() {
        let playground = sort([0, 12, 5, 4, 8].into_iter(), radix_sort_8bits);
        println!("{:?}", playground);
        assert_eq!(vec![0_i64, 4, 5, 8, 12], playground.data);
    }
}
