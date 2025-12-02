use crate::aoc_lib::echo;
use crate::aoc_lib::expect;
use colored::Colorize;

const DAY: i32 = 2;

pub fn main() {
    echo::echo_day(DAY, 1);
    echo::echo_day_example(DAY, 1);
    match example(1) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    };
    match my(1) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    }
    match example(2) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    };
    match my(2) {
        Err(e) => eprintln!("SKIPPING: {e}"),
        Ok(result) => println!("{result}"),
    }
}

fn my(part: i32) -> Result<i128, String> {
    let my_result: i128;
    match echo::get_my_day(DAY, part) {
        Err(e) => return Err(e),
        Ok(my) => {
            if let Some(result) = run_part(part, my) {
                println!("Got result: {result}");
                my_result = result
            } else {
                return Err(String::from("No output from main logic"));
            }
        }
    }
    if let Some(err) = expect::expect_my(DAY, part, my_result) {
        Err(err)
    } else {
        Ok(my_result)
    }
}

fn example(part: i32) -> Result<i128, String> {
    let my_result;
    match echo::get_day_example(DAY, part) {
        Err(e) => return Err(e),
        Ok(example) => {
            if let Some(result) = run_part(part, example) {
                println!("Got result: {result}");
                my_result = result
            } else {
                return Err(String::from("No output from main logic"));
            }
        }
    }
    if let Some(err) = expect::expect_example(DAY, part, my_result) {
        Err(err)
    } else {
        Ok(my_result)
    }
}

fn run_part(part: i32, input: String) -> Option<i128> {
    let mut invalid_sum = 0;
    println!("{invalid_sum}");
    for line in input.lines() {
        for id_range in line.split(",") {
            println!();
            if !id_range.trim().is_empty() {
                if part == 2 && let Some(sum) = validate_range2(id_range) {
                    invalid_sum += sum;
                } else if let Some(sum) = validate_range(id_range) {
                    invalid_sum += sum;
                } else {
                    eprintln!("Failed to parse {id_range}")
                }
            }
        }
    }
    Some(invalid_sum)
}

fn validate_range2(id_range: &str) -> Option<i128> {
    validate_range(id_range)
}

fn validate_range(id_range: &str) -> Option<i128> {
    let mut invalid_sum = 0;

    if let Some(ids) = id_range.split_once("-") {
        let (start, end) = ids;
        println!("range {start}-{end} ");
        match spread(String::from(start), String::from(end)) {
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
            Ok(spreaded) => {
                for id in spreaded {
                    if is_valid(&id) {
                    } else if let Some(iid) = parse_id(&id) {
                        invalid_sum += iid as i128;
                    } else {
                        eprint!("Failed to parse {id}");
                        return None;
                    }
                }
            }
        }
        Some(invalid_sum)
    } else {
        eprintln!("Failed to parse {id_range}");
        None
    }
}

fn is_valid(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return true;
    }
    let mid = id.len() / 2;
    if let Some(left_slice) = id.get(..mid) {
        if let Some(right_slice) = id.get(mid..) {
            for i in 0..mid {
                if let Some(left) = left_slice.get(i..i + 1) {
                    if let Some(right) = right_slice.get(i..i + 1) {
                        if let Some(left_char) = left.chars().next() {
                            if let Some(right_char) = right.chars().next() {
                                if left_char != right_char {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Invalid {id}");
    false
}

fn spread(start: String, end: String) -> Result<Vec<String>, String> {
    let mut index;
    match start.parse::<i128>() {
        Err(e) => return Err(format!("{e}")),
        Ok(i) => index = i,
    }
    let endindex = match end.parse::<i128>() {
        Err(e) => return Err(format!("{e}")),
        Ok(i) => i,
    };
    let mut ids = Vec::new();
    while index <= endindex {
        ids.push(index.to_string());
        index += 1;
    }
    Ok(ids)
}

fn parse_id(id: &str) -> Option<i128> {
    id.parse::<i128>().ok()
}
