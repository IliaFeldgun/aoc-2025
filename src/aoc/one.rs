use crate::aoc_lib::echo;
use crate::aoc_lib::expect;
use colored::Colorize;

const DAY: i32 = 1;

pub fn main() {
    echo::echo_day(DAY, 1);
    echo::echo_day_example(DAY, 1);
    match example_part_1() {
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
        Ok(result) => println!("{result}"),
    };
    match my_part_1() {
        Err(e) => eprintln!("{e}"),
        Ok(result) => println!("{result}"),
    }
}

fn my_part_1() -> Result<i32, String> {
    match echo::get_my_day(DAY, 1) {
        Err(e) => Err(e),
        Ok(example) => {
            if let Some(result) = part_1(example) {
                println!("Got result: {result}");
                Ok(result)
            } else {
                Err(String::from("No output from main logic"))
            }
        }
    }
}

fn example_part_1() -> Result<i32, String> {
    let my_result: i32;
    match echo::get_day_example(DAY, 1) {
        Err(e) => return Err(e),
        Ok(example) => {
            if let Some(result) = part_1(example) {
                println!("Got result: {result}");
                my_result = result
            } else {
                return Err(String::from("No output from main logic"));
            }
        }
    }
    if let Some(err) = expect::expect_example(DAY, 1, my_result) {
        Err(err)
    } else {
        Ok(my_result)
    }
}

fn part_1(input: String) -> Option<i32> {
    let init = 50;
    let mut dial = init;
    let mut total_zeroes = 0;
    print!("{dial}({total_zeroes})");
    for line in input.lines() {
        match sign_rotation(line) {
            Err(e) => println!("{e}"),
            Ok(delta) => {
                let pretty = output(dial, delta);
                dial = rotate(dial, delta);
                if dial == 0 {
                    total_zeroes += 1
                }
                println!("{pretty}={dial}    ({total_zeroes})");
            }
        }
    }
    Some(total_zeroes)
}

fn rotate(dial: i32, delta: i32) -> i32 {
    let normal_delta = delta % 100;
    if delta == 0 {
        dial
    } else {
        (dial + normal_delta) % 100
    }
}

fn output(dial: i32, delta: i32) -> String {
    let d;
    if delta < 0 {
        d = format!("{}", delta.to_string().red());
    } else {
        d = format!("+{}", delta.to_string().green());
    }
    format!("{dial}{d}")
}

fn sign_rotation(rotation: &str) -> Result<i32, String> {
    if let Some(left) = rotation.strip_prefix("L") {
        let inc = left.parse::<i32>().map_err(|e| format!("{e}"))?;
        Ok(0 - inc)
    } else if let Some(right) = rotation.strip_prefix("R") {
        right.parse::<i32>().map_err(|e| format!("{e}"))
    } else {
        Err(String::from("Neither prefix right nor left not found"))
    }
}
