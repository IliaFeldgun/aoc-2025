use std::collections::HashMap;

use crate::aoc::two::parse_id;
use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 5;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    let mut ranges: Vec<(&str, &str)> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    let mut good_ids: HashMap<i128, bool> = HashMap::new();
    for line in input.lines() {
        if line.contains("-") && let Some(ids) = line.split_once("-") {
            ranges.push(ids);
        } else if !line.trim().is_empty() {
            ids.push(line.trim());
        }
    }
    for (start, end) in ranges {
        if let Some(istart) = parse_id(start) && let Some(iend) = parse_id(end) {
            for id in ids.clone() {
                if let Some(iid) = parse_id(id) {
                    if iid <= iend && iid >= istart {
                        println!("{iid}");
                        good_ids.insert(iid, true);
                    }
                }
            }
        } else {
            eprintln!("Failed to parse range {start}-{end}");
            return None;
        }
    }
    Some(Number::Int32(good_ids.len() as i32))
}
