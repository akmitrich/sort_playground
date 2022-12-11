use dotenv::{dotenv, var};
use sort_playground::external::num_file_iter;
use std::{fs::File, io, path::Path};

fn main() -> io::Result<()> {
    dotenv().unwrap();
    let parameter_basepath = var("BASEPATH").unwrap_or_else(|_| String::from("."));
    let parameter_method = var("METHOD").unwrap_or_else(|_| String::from("naive"));
    let parameter_input = var("INPUT").unwrap_or_else(|_| String::from("input.txt"));
    let parameter_output = var("OUTPUT").unwrap_or_else(|_| String::from("output.txt"));
    let parameter_chunksize = var("CHUNKSIZE").unwrap_or_else(|_| String::from("N/A"));
    let base_path = Path::new(&parameter_basepath);
    let chunk_size: usize = parameter_chunksize.parse().unwrap_or(1000);
    let method = match parameter_method.to_lowercase().as_str() {
        "naive" => sort_playground::external::naive,
        "natural" | "twofiles" | "two_files" => sort_playground::external::two_files,
        _ => {
            println!("No such method: '{parameter_method}'");
            return Ok(());
        }
    };
    println!(
        "Start sorting file {:?}. Sorted {}%",
        base_path.join(&parameter_input),
        sorted_percent(base_path.join(&parameter_input))
    );
    method(base_path, &parameter_input, &parameter_output, chunk_size)?;
    println!(
        "Sort complete. Sorted {}% at {:?}",
        sorted_percent(base_path.join(&parameter_output)),
        base_path.join(&parameter_output),
    );
    Ok(())
}

fn sorted_percent(path: impl AsRef<Path>) -> usize {
    let Ok(sorted) = File::open(path) else {
        return 0;
    };
    let mut data = num_file_iter(&sorted);
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
    if sum > 0 {
        success * 100 / sum
    } else {
        100
    }
}
