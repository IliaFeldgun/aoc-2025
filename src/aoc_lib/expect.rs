use crate::aoc_lib::echo;

pub fn expect_example(day: i32, part: i32, calculated_result: i32) -> Option<String> {
    match echo::get_day_example_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => {
            println!("Expected: {expected}");
            if expected == calculated_result {
                None
            } else {
                Some(format!(
                    "Result {calculated_result} doesn't match with expected {expected}"
                ))
            }
        }
    }
}
