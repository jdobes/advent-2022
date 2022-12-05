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

    let mut stacks1: Vec<Vec<char>> = Vec::new();
    let mut stacks2: Vec<Vec<char>> = Vec::new();
    let mut stack_size = 0;
    for line in buffered.lines() {
        let text = line?;
        let chars: Vec<char> = text.chars().collect();

        if chars.len() > 0 && chars[0] != 'm' {
            // Init stacks
            stack_size = (chars.len()+1)/4;
            if stacks1.len() == 0 {
                for _ in 0..stack_size {
                    let s1: Vec<char> = Vec::new();
                    let s2: Vec<char> = Vec::new();
                    stacks1.push(s1);
                    stacks2.push(s2);
                }
            }

            for i in 0..stack_size {
                let c = chars[1 + i*4];
                if !c.is_whitespace() && !c.is_digit(10) {
                    stacks1[i].insert(0, c);
                    stacks2[i].insert(0, c);
                }
            }
        } else {
            let words: Vec<&str> = text.split_whitespace().collect();
            if words.len() > 0 && words[0] == "move" {
                let how_many: u32 = words[1].parse()?;
                let from_idx: usize = words[3].parse()?;
                let to_idx: usize = words[5].parse()?;
                // Stacks 1 one-by-one
                for _ in 0..how_many {
                    let c = stacks1[from_idx-1].pop().unwrap();
                    stacks1[to_idx-1].push(c);
                }
                // Stacks 2 at once
                let mut chunk: Vec<char> = Vec::new();
                for _ in 0..how_many {
                    let c = stacks2[from_idx-1].pop().unwrap();
                    chunk.insert(0, c);
                }
                for c in chunk {
                    stacks2[to_idx-1].push(c);
                }
            }
        }
    }

    //println!("{:?}", stacks);
    for i in 0..stack_size {
        print!("{}", stacks1[i].last().unwrap());
    }
    print!("\n");
    for i in 0..stack_size {
        print!("{}", stacks2[i].last().unwrap());
    }
    print!("\n");

    Ok(())
}
