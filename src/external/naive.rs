use std::{
    fs::{self, File},
    io,
    path::Path,
};

use crate::{merge::merge_sort, sort_playground::SortPlayground};

use super::{create_file_from_iter, num_file_iter, AscendingMerger};

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
            let chunk_path = path_to_chunk_number(&path, chunk_number);
            create_file_from_iter(chunk_path, chunk.data.iter().copied())?;
            chunk_number += 1;
            chunk.data.clear();
        }
    }
    let chunk_path = path_to_chunk_number(path, chunk_number);
    create_file_from_iter(chunk_path, chunk.into_iter())?;
    Ok(())
}

fn merge_sorted_chunks(path: impl AsRef<Path>) -> io::Result<()> {
    let mut chunk_number = 0_usize;
    let mut merged_number = 0_usize;
    while let Ok(left_file) = File::open(path_to_chunk_number(&path, chunk_number)) {
        let merged_path = path_to_merged_chunk_number(&path, merged_number);
        chunk_number += 1;
        if let Ok(right_file) = File::open(path_to_chunk_number(&path, chunk_number)) {
            chunk_number += 1;
            let merged =
                AscendingMerger::new(num_file_iter(&left_file), num_file_iter(&right_file));
            create_file_from_iter(merged_path, merged)?;
        } else {
            create_file_from_iter(merged_path, num_file_iter(&left_file))?;
        }
        merged_number += 1;
    }
    clean_up(&path)?;
    if merged_number > 1 {
        merge_sorted_chunks(path)?;
    }
    Ok(())
}

fn clean_up(path: impl AsRef<Path>) -> io::Result<()> {
    let mut chunk_number = 0_usize;
    while fs::remove_file(path_to_chunk_number(&path, chunk_number)).is_ok() {
        chunk_number += 1;
    }
    let mut merged_number = 0_usize;
    while fs::rename(
        path_to_merged_chunk_number(&path, merged_number),
        path_to_chunk_number(&path, merged_number),
    )
    .is_ok()
    {
        merged_number += 1;
    }
    Ok(())
}

fn path_to_chunk_number(path: impl AsRef<Path>, chunk_number: usize) -> impl AsRef<Path> {
    path.as_ref().join(format!("{}.chunk", chunk_number))
}

fn path_to_merged_chunk_number(path: impl AsRef<Path>, merged_number: usize) -> impl AsRef<Path> {
    path.as_ref().join(format!("m{}.chunk", merged_number))
}
