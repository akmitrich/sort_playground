use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use rand::Rng;

pub fn create_file_random_numbers(path: impl AsRef<Path>, n: usize, t: i64) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut rng = rand::thread_rng();
    for _ in 0..(n - 1) {
        writeln!(file, "{}", rng.gen_range(1..=t))?;
    }
    write!(file, "{}", rng.gen_range(1..=t))?;
    Ok(())
}
