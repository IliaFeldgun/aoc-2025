use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 3;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    let mut sum: i32 = 0;
    for line in input.lines() {
        if let Some(joltage) = get_max_joltage(line) {
            sum += joltage;
        } else {
            return None;
        }
    }
    Some(Number::Int32(sum))
}

fn get_max_joltage(line: &str) -> Option<i32> {
    let mut max_first: char = '0';
    let mut max_second: char = '0';
    for (index, current_char) in line.chars().enumerate() {
        if current_char > max_first && index < line.len() - 1 {
            max_first = current_char;
            max_second = '0';
        } else if current_char > max_second {
            max_second = current_char;
        }
    }
    let result = format!("{max_first}{max_second}");

    match result.trim().parse::<i32>() {
        Err(e) => {
            eprintln!("Failed to parse {result}: {e}");
            None
        }
        Ok(number) => Some(number),
    }
}
