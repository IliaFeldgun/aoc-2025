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
    let digit_collection = get_max_joltage(line, digits);
    let mut result = String::from("");
    result = format!("{result}{digit_collection}");
    match result.trim().parse::<i128>() {
        Err(e) => {
            eprintln!("Failed to parse {result}: {e}");
            None
        }
        Ok(number) => Some(number),
    }
}

fn get_max_joltage(line: &str, digits: usize) -> String {
    let mut line_clone = line;
    let mut digit_collection = format!("{:0>digits$}", "");
    println!("{digit_collection}");
    let mut max = String::new();
    for _ in 0..digits {
        max = format!("{digit_collection}9")
    }
    let mut min_digit = 0;
    while !line_clone.is_empty() && digit_collection != max {
        if let Some(first_in_line) = line_clone.chars().next() {
            for digit in min_digit..digits {
                if let Some(max_first) = digit_collection.chars().nth(digit) {
                    if first_in_line > max_first && line_clone.len() >= digits - digit {
                        let slice: String = digit_collection.chars().take(digit).collect();
                        digit_collection =
                            format!("{:0<digits$}", format!("{slice}{first_in_line}"));
                        if first_in_line == '9' {
                            min_digit += 1;
                            break;
                        }
                        break;
                    }
                }
            }
        }
        line_clone = &line_clone[1..];
    }
    digit_collection
}
