use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

fn get_prio(ch: &char) -> u32 {
    let mut val = *ch as u32;
    if val >= 65 && val <= 90 {
        val -= 38;
    }
    else if val >= 97 && val <= 122 {
        val -= 96;
    }
    //println!("{} -> {}", ch, val);
    return val
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

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;

    let mut group_cnt = 0;
    let mut group_types = HashSet::new();

    for line in buffered.lines() {
        let text = line?;
        let chars = text.chars().collect::<Vec<char>>();
        let rucksack_size = chars.len();
        let compartment_size = rucksack_size/2;
        let mut rucksack_types = HashSet::new();
        let mut first_compartment_types = HashSet::new();
        let mut second_compartment_types = HashSet::new();

        for i in 0..compartment_size {
            rucksack_types.insert(chars[i]);
            first_compartment_types.insert(chars[i]);
        }
        for i in compartment_size..rucksack_size {
            rucksack_types.insert(chars[i]);
            second_compartment_types.insert(chars[i]);
        }
        for c in first_compartment_types.intersection(&second_compartment_types) {
            sum1 += get_prio(c);
        }

        if group_types.len() == 0 {
            // First rucksack in group, no intersection, just copy
            for c in rucksack_types {
                group_types.insert(c);
            }
        } else {
            let mut new_group_types = HashSet::new();
            for c in group_types.intersection(&rucksack_types) {
                new_group_types.insert(*c);
            }
            group_types = new_group_types;
        }

        if group_cnt >= 2 {
            // Flush group
            for c in group_types {
                sum2 += get_prio(&c);
            }
            group_cnt = 0;
            group_types = HashSet::new();
        } else {
            group_cnt += 1;
        }
    }

    println!("{}", sum1);
    println!("{}", sum2);

    Ok(())
}
