use std::collections::HashSet;
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

    let mut heights: Vec<i32> = Vec::new();
    let mut edge_len: u32 = 0;
    for line in buffered.lines() {
        let text = line?;
        let chars: Vec<char> = text.chars().collect();
        for ch in &chars {
            let height: i32 = ch.to_digit(10).unwrap() as i32;
            heights.push(height);
        }
        edge_len = chars.len() as u32;
    }

    let max_heights_idx = heights.len() as usize - 1;
    let mut visible_indexes = HashSet::new();
    let mut highest_in_line: i32 = -1;
    let mut highest_scenic_score: u32 = 0;

    for i in 0..edge_len {
        let left_start = i * edge_len;
        let right_start = i * edge_len + edge_len - 1;
        let top_start = i;
        let bottom_start = edge_len * (edge_len - 1) + i;

        // From left
        for j in 0..edge_len {
            let idx: usize = left_start as usize + j as usize;
            if heights[idx] > highest_in_line {
                visible_indexes.insert(idx);
                highest_in_line = heights[idx];
            }
        }
        highest_in_line = -1;

        // From right
        for j in 0..edge_len {
            let idx: usize = right_start as usize - j as usize;
            if heights[idx] > highest_in_line {
                visible_indexes.insert(idx);
                highest_in_line = heights[idx];
            }
        }
        highest_in_line = -1;

        // From top
        for j in 0..edge_len {
            let idx: usize = top_start as usize + j as usize * edge_len as usize;
            if heights[idx] > highest_in_line {
                visible_indexes.insert(idx);
                highest_in_line = heights[idx];
            }
        }
        highest_in_line = -1;

        // From bottom
        for j in 0..edge_len {
            let idx: usize = bottom_start as usize - j as usize * edge_len as usize;
            if heights[idx] > highest_in_line {
                visible_indexes.insert(idx);
                highest_in_line = heights[idx];
            }
        }
        highest_in_line = -1;

        // Compute scenic score
        for j in 0..edge_len {
            let idx: usize = left_start as usize + j as usize;
            let mut can_see_left: u32 = 0;
            let mut can_see_right: u32 = 0;
            let mut can_see_up: u32 = 0;
            let mut can_see_down: u32 = 0;

            // Check left side
            let mut jdx = idx;
            loop {
                if (jdx as i32 - 1) < left_start as i32 {
                    break;
                }
                jdx -= 1;
                can_see_left += 1;
                if heights[jdx] >= heights[idx] {
                    break
                }
            }

            // Check right side
            jdx = idx;
            loop {
                if (jdx as i32 + 1) > right_start as i32 {
                    break;
                }
                jdx += 1;
                can_see_right += 1;
                if heights[jdx] >= heights[idx] {
                    break
                }
            }

            // Check up side
            jdx = idx;
            loop {
                if (jdx as i32 - edge_len as i32) < 0 {
                    break;
                }
                jdx -= edge_len as usize;
                can_see_up += 1;
                if heights[jdx] >= heights[idx] {
                    break
                }
            }

            // Check down side
            jdx = idx;
            loop {
                if (jdx as i32 + edge_len as i32) > max_heights_idx as i32 {
                    break;
                }
                jdx += edge_len as usize;
                can_see_down += 1;
                if heights[jdx] >= heights[idx] {
                    break
                }
            }

            let scenic_score: u32 = can_see_left * can_see_right * can_see_up * can_see_down;
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!("{:?}", visible_indexes.len());
    println!("{:?}", highest_scenic_score);

    Ok(())
}
