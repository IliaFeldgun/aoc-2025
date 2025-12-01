use crate::aoc_lib::echo;

pub fn expect_example(day: i32, part: i32, calculated_result: i32) -> Option<String> {
    match echo::get_day_example_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected, calculated_result),
    }
}

pub fn expect_my(day: i32, part: i32, calculated_result: i32) -> Option<String> {
    match echo::get_my_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected, calculated_result),
    }
}

fn expect(expected: i32, calculated: i32) -> Option<String> {
    println!("Expected: {expected}");
    if expected == calculated {
        None
    } else {
        Some(format!(
            "Result {calculated} doesn't match with expected {expected}"
        ))
    }
}
