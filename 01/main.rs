use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path.txt>", args[0]);
        exit(1);
    }

    let filepath = &args[1];
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    let mut values: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in buffered.lines() {
        let text = line?;
        if text.is_empty() {
            values.push(sum);
            sum = 0;
        } else {
            let number: i32 = text.parse()?;
            sum += number;
        }
    }
    if sum > 0 {
        values.push(sum);
    }

    values.sort();
    values.reverse();

    println!("{}", values[0]);
    println!("{}", values[0] + values[1] + values[2]);
    Ok(())
}
