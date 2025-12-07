use std::collections::HashMap;

use crate::aoc::two::parse_id;
use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 5;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    if part == 1 && let Some(count) = run_part_1(input.clone()) {
        return Some(Number::Int32(count));
    } else if let Some(count) = run_part_2(input.clone()) {
        return Some(Number::Int128(count));
    }
    None
}

fn run_part_2(input: String) -> Option<i128> {
    let mut ranges: Vec<(&str, &str)> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    let mut good_ranges: HashMap<i128, i128> = HashMap::new();
    for line in input.lines() {
        if line.contains("-") && let Some(ids) = line.split_once("-") {
            ranges.push(ids);
        } else if !line.trim().is_empty() {
            ids.push(line.trim());
        }
    }
    for (start, end) in ranges {
        if let Some(istart) = parse_id(start) && let Some(iend) = parse_id(end) {
            let mut new_start = istart;
            let mut new_end = iend;
            let mut remove_indices: Vec<i128> = Vec::new();
            for (good_start, good_end) in good_ranges.clone() {
                if new_end < good_start - 1 {
                    continue;
                }
                if new_start > good_end + 1 {
                    continue;
                }
                if new_end <= good_end {
                    new_end = good_end;
                }
                if good_start <= new_start {
                    new_start = good_start;
                } else {
                    remove_indices.push(good_start);
                }
            }
            for i in remove_indices {
                good_ranges.remove(&i);
            }
            good_ranges.insert(new_start, new_end);
        } else {
            eprintln!("Failed to parse range {start}-{end}");
            return None;
        }
    }
    let mut count: i128 = 0;
    for (good_start, good_end) in good_ranges.clone() {
        count += good_end - good_start + 1
    }
    Some(count)
}
fn run_part_1(input: String) -> Option<i32> {
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
                        good_ids.insert(iid, true);
                    }
        }
            }
        } else {
            eprintln!("Failed to parse range {start}-{end}");
            return None;
        }
    }
    Some(good_ids.len() as i32)
}
