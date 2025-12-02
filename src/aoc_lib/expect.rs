use crate::aoc_lib::echo;

enum Number {
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

pub fn expect_example(day: i32, part: i32, calculated_result: i128) -> Option<String> {
    match echo::get_day_example_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected as i128, calculated_result),
    }
}

pub fn expect_my(day: i32, part: i32, calculated_result: i128) -> Option<String> {
    match echo::get_my_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected as i128, calculated_result),
    }
}

fn expect(expected: i128, calculated: i128) -> Option<String> {
    println!("Expected: {expected}");
    if expected == calculated {
        None
    } else {
        Some(format!(
            "Result {calculated} doesn't match with expected {expected}"
        ))
    }
}
