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

    let mut contained: u32 = 0;
    let mut overlaps: u32 = 0;
    for line in buffered.lines() {
        let text = line?;
        let pair: Vec<&str> = text.split(",").collect();
        let mut intervals = Vec::new();
        for elf in pair {
            let parts: Vec<&str> = elf.split("-").collect();
            let begin: u32 = parts[0].parse()?;
            let end: u32 = parts[1].parse()?;
            let interval = vec![begin, end];
            intervals.push(interval);
        }
        if (intervals[0][0] >= intervals[1][0] && intervals[0][1] <= intervals[1][1]) ||
           (intervals[1][0] >= intervals[0][0] && intervals[1][1] <= intervals[0][1]) {
            contained += 1;
        }
        if (intervals[1][0] >= intervals[0][0] && intervals[1][0] <= intervals[0][1]) ||
           (intervals[0][0] >= intervals[1][0] && intervals[0][0] <= intervals[1][1]) {
            overlaps += 1;
        }
    }

    println!("{}", contained);
    println!("{}", overlaps);

    Ok(())
}
