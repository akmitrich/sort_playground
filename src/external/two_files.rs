use std::{
    cmp::Ordering,
    fs::{remove_file, rename, File},
    io::{self, Write},
    path::Path,
};

use crate::{merge::merge_sort, sort_playground::SortPlayground};

use super::num_file_iter;

pub fn two_files(
    path: impl AsRef<Path>,
    input: &str,
    output: &str,
    chunk_size: usize,
) -> io::Result<()> {
    let input = File::open(path.as_ref().join(input))?;
    let input_stream = num_file_iter(&input);
    start_ab_files(&path, input_stream, chunk_size)?;
    loop {
        let sorted = merge_ab_files(&path)?;
        clean_up(&path)?;
        if sorted {
            break;
        }
    }
    rename(path.as_ref().join("a.part"), path.as_ref().join(output))?;
    remove_file(path.as_ref().join("b.part"))?;
    Ok(())
}

fn start_ab_files(
    path: impl AsRef<Path>,
    data: impl Iterator<Item = i64>,
    chunk_size: usize,
) -> io::Result<()> {
    let mut ab = false;
    let mut a_file = File::create(a_file_path(&path))?;
    let mut b_file = File::create(b_file_path(&path))?;
    let mut chunk = SortPlayground::default();
    for n in data {
        if chunk.data.len() == chunk_size {
            chunk = sort_then_append(if ab { &mut b_file } else { &mut a_file }, chunk)?;
            ab = !ab;
        }
        chunk.data.push(n);
    }
    if !chunk.data.is_empty() {
        sort_then_append(if ab { &mut b_file } else { &mut a_file }, chunk)?;
    }
    Ok(())
}

fn sort_then_append(ab: &mut File, mut chunk: SortPlayground) -> io::Result<SortPlayground> {
    chunk = merge_sort(chunk);
    write_iter(ab, chunk.data.iter().copied())?;
    chunk.data.clear();
    Ok(chunk)
}

fn merge_ab_files(path: impl AsRef<Path>) -> io::Result<bool> {
    let mut sorted = true;
    let mut mn = false;
    let m_file = File::create(m_file_path(&path))?;
    let n_file = File::create(n_file_path(&path))?;
    let a_file = File::open(a_file_path(&path))?;
    let b_file = File::open(b_file_path(&path))?;
    let mut a_data = num_file_iter(&a_file).peekable();
    let mut b_data = num_file_iter(&b_file).peekable();
    let mut prev_a = i64::MIN;
    let mut prev_b = i64::MIN;
    loop {
        let order = match (a_data.peek(), b_data.peek()) {
            (None, None) => None,
            (None, Some(b)) if &prev_b > b => {
                mn = !mn;
                sorted = false;
                prev_b = *b;
                continue;
            }
            (Some(a), None) if &prev_a > a => {
                mn = !mn;
                sorted = false;
                prev_a = *a;
                continue;
            }
            (None, Some(_)) => Some(Ordering::Greater),
            (Some(_), None) => Some(Ordering::Less),
            (Some(a), Some(b)) => match (prev_a.cmp(a), prev_b.cmp(b)) {
                (Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                    Some(a.cmp(b))
                }
                (Ordering::Less | Ordering::Equal, Ordering::Greater) => Some(Ordering::Less),
                (Ordering::Greater, Ordering::Less | Ordering::Equal) => Some(Ordering::Greater),
                (Ordering::Greater, Ordering::Greater) => {
                    mn = !mn;
                    sorted = false;
                    prev_a = *a;
                    prev_b = *b;
                    continue;
                }
            },
        };
        let current = match order {
            Some(Ordering::Equal | Ordering::Less) => {
                prev_a = a_data.next().unwrap();
                prev_a
            }
            Some(Ordering::Greater) => {
                prev_b = b_data.next().unwrap();
                prev_b
            }
            None => break,
        };
        writeln!(if mn { &n_file } else { &m_file }, "{current}")?;
    }
    Ok(sorted)
}

fn clean_up(path: impl AsRef<Path>) -> io::Result<()> {
    if a_file_path(&path).as_ref().is_file() {
        remove_file(a_file_path(&path))?;
    }
    if b_file_path(&path).as_ref().is_file() {
        remove_file(b_file_path(&path))?;
    }
    if m_file_path(&path).as_ref().is_file() {
        rename(m_file_path(&path), a_file_path(&path))?;
    }
    if n_file_path(&path).as_ref().is_file() {
        rename(n_file_path(&path), b_file_path(&path))?;
    }
    Ok(())
}

fn write_iter(file: &mut File, chunk: impl Iterator<Item = i64>) -> io::Result<()> {
    for number in chunk {
        writeln!(file, "{number}")?;
    }
    Ok(())
}

fn a_file_path(path: impl AsRef<Path>) -> impl AsRef<Path> {
    path.as_ref().join("a.part")
}

fn b_file_path(path: impl AsRef<Path>) -> impl AsRef<Path> {
    path.as_ref().join("b.part")
}

fn m_file_path(path: impl AsRef<Path>) -> impl AsRef<Path> {
    path.as_ref().join("m.part")
}

fn n_file_path(path: impl AsRef<Path>) -> impl AsRef<Path> {
    path.as_ref().join("n.part")
}
