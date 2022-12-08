use dotenv::{dotenv, var};
use sort_playground::external::create_file_random_numbers;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    dotenv().unwrap();
    let parameter_basepath = var("BASEPATH").unwrap_or_else(|_| String::from("."));
    let parameter_method = var("METHOD").unwrap_or_else(|_| String::from("naive"));
    let parameter_input = var("INPUT").unwrap_or_else(|_| String::from("input.txt"));
    let parameter_output = var("OUTPUT").unwrap_or_else(|_| String::from("output.txt"));
    let parameter_chunksize = var("CHUNKSIZE").unwrap_or_else(|_| String::from("N/A"));
    let base_path = Path::new(&parameter_basepath);
    let chunk_size: usize = parameter_chunksize.parse().unwrap_or(1000);
    println!("{parameter_method:?} {base_path:?} {chunk_size}");
    Ok(())
}
