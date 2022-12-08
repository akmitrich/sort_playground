use std::{
    cmp::Ordering,
    fs::{self, File},
    io::{self, BufRead, Write},
    iter::Peekable,
    path::Path,
};

use rand::Rng;

use crate::{merge::merge_sort, sort_playground::SortPlayground};

pub fn create_file_random_numbers(path: impl AsRef<Path>, n: usize, t: i64) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut rng = rand::thread_rng();
    for _ in 0..(n - 1) {
        writeln!(file, "{}", rng.gen_range(1..=t))?;
    }
    write!(file, "{}", rng.gen_range(1..=t))?;
    Ok(())
}

pub fn naive(
    path: impl AsRef<Path>,
    input: &str,
    output: &str,
    chunk_size: usize,
) -> io::Result<()> {
    let input = File::open(path.as_ref().join(input))?;
    create_sorted_chunks(&path, &input, chunk_size)?;
    merge_sorted_chunks(&path)?;
    fs::rename(path.as_ref().join("0.chunk"), path.as_ref().join(output))?;
    Ok(())
}

fn create_sorted_chunks(path: impl AsRef<Path>, input: &File, chunk_size: usize) -> io::Result<()> {
    let mut chunk = SortPlayground::default();
    let mut chunk_number = 0_usize;
    for n in num_file_iter(input) {
        chunk.data.push(n);
        if chunk.data.len() == chunk_size {
            chunk = merge_sort(chunk);
            let chunk_path = path.as_ref().join(format!("{chunk_number}.chunk"));
            create_file_from_iter(chunk_path, chunk.data.iter().copied())?;
            chunk_number += 1;
            chunk.data.clear();
        }
    }
    let chunk_path = path.as_ref().join(format!("{chunk_number}.chunk"));
    create_file_from_iter(chunk_path, chunk.into_iter())?;
    Ok(())
}

fn merge_sorted_chunks(path: impl AsRef<Path>) -> io::Result<()> {
    let mut chunk_number = 0_usize;
    let mut merged_number = 0_usize;
    while let Ok(left_file) = File::open(path.as_ref().join(format!("{}.chunk", chunk_number))) {
        chunk_number += 1;
        if let Ok(right_file) = File::open(path.as_ref().join(format!("{}.chunk", chunk_number))) {
            chunk_number += 1;
            let merged = Ascending::new(num_file_iter(&left_file), num_file_iter(&right_file));
            create_file_from_iter(
                path.as_ref().join(format!("m{merged_number}.chunk")),
                merged,
            )?;
        } else {
            create_file_from_iter(
                path.as_ref().join(format!("m{merged_number}.chunk")),
                num_file_iter(&left_file),
            )?;
        }
        merged_number += 1;
    }
    clean_up(&path)?;
    if merged_number > 1 {
        merge_sorted_chunks(path)?;
    }
    Ok(())
}

fn num_file_iter(input: &File) -> impl Iterator<Item = i64> + '_ {
    io::BufReader::new(input)
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<i64>().ok())
}

fn create_file_from_iter(
    path: impl AsRef<Path>,
    values: impl Iterator<Item = i64>,
) -> io::Result<()> {
    let mut file = File::create(path)?;
    for n in values {
        writeln!(file, "{n}")?;
    }
    Ok(())
}

fn clean_up(path: impl AsRef<Path>) -> io::Result<()> {
    let mut chunk_number = 0_usize;
    while fs::remove_file(path.as_ref().join(format!("{}.chunk", chunk_number))).is_ok() {
        chunk_number += 1;
    }
    let mut merged_number = 0_usize;
    while fs::rename(
        path.as_ref().join(format!("m{}.chunk", merged_number)),
        path.as_ref().join(format!("{}.chunk", merged_number)),
    )
    .is_ok()
    {
        merged_number += 1;
    }
    Ok(())
}

struct Ascending<I: Iterator<Item = i64>> {
    left: Peekable<I>,
    right: Peekable<I>,
}

impl<I: Iterator<Item = i64>> Ascending<I> {
    pub fn new(left: I, right: I) -> Self {
        Self {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<I: Iterator<Item = i64>> Iterator for Ascending<I> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let order = match (self.left.peek(), self.right.peek()) {
            (None, None) => None,
            (None, Some(_)) => Some(Ordering::Greater),
            (Some(_), None) => Some(Ordering::Less),
            (Some(l), Some(r)) => Some(l.cmp(r)),
        };
        match order {
            Some(Ordering::Equal | Ordering::Less) => self.left.next(),
            Some(Ordering::Greater) => self.right.next(),
            None => None,
        }
    }
}
