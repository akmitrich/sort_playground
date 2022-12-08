use dotenv::{dotenv, var};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> io::Result<()> {
    dotenv().unwrap();
    let parameter_basepath = var("BASEPATH").unwrap_or_else(|_| String::from("."));
    let _parameter_method = var("METHOD").unwrap_or_else(|_| String::from("naive"));
    let parameter_input = var("INPUT").unwrap_or_else(|_| String::from("input.txt"));
    let parameter_output = var("OUTPUT").unwrap_or_else(|_| String::from("output.txt"));
    let parameter_chunksize = var("CHUNKSIZE").unwrap_or_else(|_| String::from("N/A"));
    let base_path = Path::new(&parameter_basepath);
    let chunk_size: usize = parameter_chunksize.parse().unwrap_or(1000);
    match sort_playground::external::naive(
        base_path,
        &parameter_input,
        &parameter_output,
        chunk_size,
    ) {
        Ok(_) => {
            println!(
                "Sort complete. Sorted {}%",
                sorted_percent(base_path.join(parameter_output))?
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn sorted_percent(path: impl AsRef<Path>) -> io::Result<usize> {
    let sorted = File::open(path)?;
    let mut data = BufReader::new(sorted)
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<i64>().ok());
    let start = data.next().unwrap();
    let (sum, success, _) = data.fold((0, 0, start), |(sum, success, prev), current| {
        (
            sum + 1,
            if prev <= current {
                success + 1
            } else {
                success
            },
            current,
        )
    });
    Ok(if sum > 0 { success * 100 / sum } else { 100 })
}
