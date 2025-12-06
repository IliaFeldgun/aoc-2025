use crate::aoc_lib::expect::Number;
use crate::aoc_lib::{echo, expect};

pub fn main(day: i32, aoc_part: fn(i32, String) -> Option<Number>) {
    echo::echo_day(day, 1);
    echo::echo_day_example(day, 1);
    match example(day, 1, aoc_part) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    };
    match my(day, 1, aoc_part) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    }
    echo::echo_day(day, 2);
    match example(day, 2, aoc_part) {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    };
    match my(day, 2, aoc_part) {
        Err(e) => eprintln!("SKIPPING: {e}"),
        Ok(result) => println!("{result}"),
    }
}

pub fn my(
    day: i32,
    part: i32,
    aoc_part: fn(i32, String) -> Option<Number>,
) -> Result<Number, String> {
    let my_result;
    match echo::get_my_day(day, part) {
        Err(e) => return Err(e),
        Ok(my) => {
            if let Some(result) = aoc_part(part, my) {
                println!("Got result: {result}");
                my_result = result
            } else {
                return Err(String::from("No output from main logic"));
            }
        }
    }
    if let Some(err) = expect::expect_my(day, part, my_result.clone()) {
        Err(err)
    } else {
        Ok(my_result)
    }
}

pub fn example(
    day: i32,
    part: i32,
    aoc_part: fn(i32, String) -> Option<Number>,
) -> Result<Number, String> {
    let my_result;
    match echo::get_day_example(day, part) {
        Err(e) => return Err(e),
        Ok(example) => {
            if let Some(result) = aoc_part(part, example) {
                println!("Got result: {result}");
                my_result = result
            } else {
                return Err(String::from("No output from main logic"));
            }
        }
    }
    if let Some(err) = expect::expect_example(day, part, my_result.clone()) {
        Err(err)
    } else {
        Ok(my_result)
    }
}
