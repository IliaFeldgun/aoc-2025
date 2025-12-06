use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 3;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    let mut sum: i128 = 0;
    for line in input.lines() {
        if part == 1 && let Some(joltage) = get_max_joltage_2(line, 2) {
            println!("{line}\n{sum}+={joltage}");
            sum += joltage;
        } else if let Some(joltage) = get_max_joltage_2(line, 12) {
            println!("{line}\n{sum}+={joltage}");
            sum += joltage;
        } else {
            return None;
        }
    }
    Some(Number::Int128(sum))
}
fn get_max_joltage_2(line: &str, digits: usize) -> Option<i128> {
    let mut digit_collection: Vec<char> = vec!['0'; digits];
    let max_line_index = line.len() - digits + 1;
    for digit_index in 0..(digits - 1) {
        for (line_index, current_char) in line.chars().enumerate() {
            // let (max_first, max_second) = get_max_joltage_1_chars(&line[line_index..]);
            let (max_first, max_second) = get_max_joltage_1_chars(line);
            digit_collection[digit_index] = max_first;
            digit_collection[digit_index + 1] = max_second;
        }
    }
    let mut result = String::from("");
    for digit in digit_collection {
        result = format!("{result}{digit}")
    }
    match result.trim().parse::<i128>() {
        Err(e) => {
            eprintln!("Failed to parse {result}: {e}");
            None
        }
        Ok(number) => Some(number),
    }
}

fn get_max_joltage_1_chars(line: &str) -> (char, char) {
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
    (max_first, max_second)
}
