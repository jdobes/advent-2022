use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::exit;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
enum YourResult {
    Lost,
    Draw = 3,
    Won = 6,
}

fn parse_move(move_char: &str) -> Move {
    if move_char == "A" || move_char == "X" {
        return Move::Rock;
    } else if move_char == "B" || move_char == "Y" {
        return Move::Paper;
    } else if move_char == "C" || move_char == "Z" {
        return Move::Scissors;
    }
    eprintln!("Unknown move: {}", move_char);
    exit(1);
}

fn parse_expected_result(result_char: &str) -> YourResult {
    if result_char == "X" {
        return YourResult::Lost;
    } else if result_char == "Y" {
        return YourResult::Draw;
    } else if result_char == "Z" {
        return YourResult::Won;
    }
    eprintln!("Unknown result: {}", result_char);
    exit(1);
}

fn get_your_score_1(opponent_move: Move, your_move: Move) -> u8 {
    let mut result = YourResult::Lost;
    if your_move == opponent_move {
        result = YourResult::Draw;
    }
    if (opponent_move == Move::Rock && your_move == Move::Paper) ||
       (opponent_move == Move::Paper && your_move == Move::Scissors) ||
       (opponent_move == Move::Scissors && your_move == Move::Rock) {
        result = YourResult::Won;
    }
    return your_move as u8 + result as u8
}

fn get_your_score_2(opponent_move: Move, expected_result: YourResult) -> u8 {
    let mut your_move = opponent_move;
    if opponent_move == Move::Rock {
        if expected_result == YourResult::Lost {
            your_move = Move::Scissors;
        } else if expected_result == YourResult::Won {
            your_move = Move::Paper
        }
    }
    else if opponent_move == Move::Paper {
        if expected_result == YourResult::Lost {
            your_move = Move::Rock;
        } else if expected_result == YourResult::Won {
            your_move = Move::Scissors
        }
    }
    else if opponent_move == Move::Scissors {
        if expected_result == YourResult::Lost {
            your_move = Move::Paper;
        } else if expected_result == YourResult::Won {
            your_move = Move::Rock
        }
    }
    return your_move as u8 + expected_result as u8
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

    let mut total_score_1: u32 = 0;
    let mut total_score_2: u32 = 0;
    for line in buffered.lines() {
        let text = line?;
        let chars: Vec<&str> = text.split_whitespace().collect();
        if chars.len() != 2 {
            eprintln!("Row doesn't have 2 parts: {:?}", chars);
            exit(1);
        }

        let opponent_move = parse_move(chars[0]);
        let your_move = parse_move(chars[1]);
        let your_score_1 = get_your_score_1(opponent_move, your_move);
        total_score_1 += your_score_1 as u32;

        let expected_result = parse_expected_result(chars[1]);
        let your_score_2 = get_your_score_2(opponent_move, expected_result);
        total_score_2 += your_score_2 as u32;
    }

    println!("{}", total_score_1);
    println!("{}", total_score_2);
    Ok(())
}
