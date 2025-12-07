use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 2;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    let mut invalid_sum = 0;
    println!("{invalid_sum}");
    for line in input.lines() {
        for id_range in line.split(",") {
            if !id_range.trim().is_empty() {
                if let Some(sum) = validate_range(part, id_range) {
                    invalid_sum += sum;
                } else {
                    eprintln!("Failed to parse {id_range}")
                }
            }
        }
    }
    Some(Number::Int128(invalid_sum))
}

fn validate_range(part: i32, id_range: &str) -> Option<i128> {
    let mut invalid_sum = 0;

    if let Some(ids) = id_range.split_once("-") {
        let (start, end) = ids;
        // println!("range {start}-{end} ");
        match spread(start, end) {
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
            Ok(spreaded) => {
                for id in spreaded {
                    let is_valid = if part == 2 {
                        is_valid_2(&id)
                    } else {
                        is_valid(&id)
                    };
                    if is_valid {
                    } else if let Some(iid) = parse_id(&id) {
                        // println!("invalid {id}");
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
            if left_slice == right_slice {
                return false;
            }
        }
    }
    true
}

fn is_valid_2(id: &str) -> bool {
    if id.len() == 1 {
        return true;
    }
    if !is_valid(id) {
        return false;
    }
    let mut pattern = String::from("");
    for (i, digit) in id.chars().enumerate() {
        pattern.push(digit);
        if id.replace(&pattern, "").is_empty() {
            return false;
        } else if i >= id.len() / 2 - 1 {
            return true;
        }
    }
    true
}

fn spread(start: &str, end: &str) -> Result<Vec<String>, String> {
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

pub fn parse_id(id: &str) -> Option<i128> {
    id.parse::<i128>().ok()
}
