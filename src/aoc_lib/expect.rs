use crate::aoc_lib::echo;
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub enum Number {
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

impl From<Number> for i128 {
    fn from(num: Number) -> Self {
        match num {
            Number::Int32(value) => value as i128,
            Number::Int64(value) => value as i128,
            Number::Int128(value) => value,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Int32(value) => write!(f, "{value}"),
            Number::Int64(value) => write!(f, "{value}"),
            Number::Int128(value) => write!(f, "{value}"),
        }
    }
}

pub fn expect_example(day: i32, part: i32, calculated_result: Number) -> Option<String> {
    match echo::get_day_example_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected, calculated_result),
    }
}

pub fn expect_my(day: i32, part: i32, calculated_result: Number) -> Option<String> {
    match echo::get_my_result(day, part) {
        Err(e) => Some(format!("Failed to get expected answer {e}")),
        Ok(expected) => expect(expected, calculated_result),
    }
}

fn expect(expected: Number, calculated: Number) -> Option<String> {
    println!("Expected  : {expected}");
    if i128::from(expected.clone()) == i128::from(calculated.clone()) {
        None
    } else {
        Some(format!(
            "Result {calculated} doesn't match with expected {expected}"
        ))
    }
}
