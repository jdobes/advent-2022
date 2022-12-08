use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

#[derive(Clone)]
struct DirFile {
    size: u32,
}

#[derive(Clone)]
struct Dir {
    subdir_paths: Vec<String>,
    files: Vec<DirFile>,
}

fn get_dir_total_size(dirs: &HashMap<String, Dir>, dir: &Dir) -> u32 {
    let mut total_size: u32 = 0;
    for subdir_path in &dir.subdir_paths {
        let subdir = dirs.get(subdir_path).unwrap();
        total_size += get_dir_total_size(dirs, subdir);
    }
    for file in &dir.files {
        total_size += file.size;
    }
    return total_size
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

    let mut cur_path: Vec<String> = Vec::new();
    let mut cur_path_key: String = String::from("");
    let mut dirs: HashMap<String, Dir> = HashMap::new();

    for line in buffered.lines() {
        let text = line?;
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() > 0 {
            if words[0] == "$" { // Command
                if words[1] == "cd" {
                    if words[2] == ".." {
                        cur_path.pop();
                    } else {
                        cur_path.push(words[2].to_string());
                    }
                    cur_path_key = cur_path.join("_");
                    if !dirs.contains_key(&cur_path_key) { // Need to create dir
                        let new_dir: Dir = Dir{files: Vec::new(), subdir_paths: Vec::new()};
                        dirs.insert(cur_path_key.to_string(), new_dir);
                    }
                }
            } else if words[0] == "dir" { // Dir listing
                // Check if dir is known, create if not
                let dir_name = words[1].to_string();
                let mut dir_path = cur_path.clone();
                dir_path.push(dir_name.to_string());
                let dir_path_key = dir_path.join("_");
                if !dirs.contains_key(&dir_path_key) { // Need to create dir
                    let new_dir: Dir = Dir{files: Vec::new(), subdir_paths: Vec::new()};
                    dirs.insert(dir_path_key.to_string(), new_dir);
                    // Also insert dir name reference to current dir
                    let cur_dir: &mut Dir = dirs.get_mut(&cur_path_key).unwrap();
                    cur_dir.subdir_paths.push(dir_path_key.to_string());
                }
            } else { // File listing
                let cur_dir: &mut Dir = dirs.get_mut(&cur_path_key).unwrap();
                let file_size: u32 = words[0].parse()?;
                let new_file: DirFile = DirFile{size: file_size};
                cur_dir.files.push(new_file);
            }
        }
    }


    let filesystem_space: u32 = 70000000;
    let required_space: u32 = 30000000;

    let mut sum_of_smaller_than_100000: u32 = 0;
    let mut smallest_dir_to_delete: u32 = std::u32::MAX;

    let mut dir_sizes: Vec<u32> = Vec::new();

    // Calculate total dir size, ineffective, looping through some dirs multiple times
    for (_, dir) in &dirs {
        dir_sizes.push(get_dir_total_size(&dirs, &dir));
    }

    let filesystem_used: u32 = *dir_sizes.iter().max().unwrap();

    let cur_free_space: u32 = filesystem_space - filesystem_used;
    let need_to_free: u32 = required_space - cur_free_space;

    for dir_size in dir_sizes {
        if dir_size <= 100000 {
            sum_of_smaller_than_100000 += dir_size;
        }
        if dir_size >= need_to_free && dir_size < smallest_dir_to_delete {
            smallest_dir_to_delete = dir_size;
        }
    }

    println!("{}", sum_of_smaller_than_100000);
    println!("{}", smallest_dir_to_delete);

    Ok(())
}
