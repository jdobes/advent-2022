use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn find_start_marker(chars: &Vec<char>, window_size: usize) -> u32 {
    let mut found_at: u32 = 0;
    for start_i in 0..chars.len()-window_size-1 {
        let end_i = start_i + window_size;
        let mut uniq = HashSet::new();
        for i in start_i..end_i {
            uniq.insert(chars[i]);
        }
        if uniq.len() == window_size {
            found_at = end_i as u32;
            break;
        }
    }
    return found_at;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path.txt>", args[0]);
        exit(1);
    }

    let filepath = &args[1];
    let file = File::open(filepath)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        let text = line?;
        let chars: Vec<char> = text.chars().collect();

        let mut sop_found_at: u32 = 0;
        let mut som_found_at: u32 = 0;

        if chars.len() >= 14 {
            sop_found_at = find_start_marker(&chars, 4);
            som_found_at = find_start_marker(&chars, 14);
        } else {
            println!("At least 14 chars are needed in the input message.");
        }

        if sop_found_at > 0 {
            println!("{}", sop_found_at);
        } else {
            println!("Start of packet marker not found in datastream.");
        }

        if som_found_at > 0 {
            println!("{}", som_found_at);
        } else {
            println!("Start of message marker not found in datastream.");
        }
    }

    Ok(())
}
