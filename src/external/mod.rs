use rand::Rng;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, Write},
    iter::Peekable,
    path::Path,
    time::Instant,
};

mod naive;

pub use naive::naive;

pub fn create_file_random_numbers(path: impl AsRef<Path>, n: usize, t: i64) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    for i in 0..(n - 1) {
        writeln!(file, "{}", rng.gen_range(1..=t))?;
        if i % 1_000_000 == 0 {
            println!(
                "Prepared {i} numbers. Passed {:?}...",
                Instant::now().duration_since(start)
            );
        }
    }
    write!(file, "{}", rng.gen_range(1..=t))?;
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

struct AscendingMerger<I: Iterator<Item = i64>> {
    left: Peekable<I>,
    right: Peekable<I>,
}

impl<I: Iterator<Item = i64>> AscendingMerger<I> {
    pub fn new(left: I, right: I) -> Self {
        Self {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<I: Iterator<Item = i64>> Iterator for AscendingMerger<I> {
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
