use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;
use colored::Colorize;

const DAY: i32 = 1;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    let init = 50;
    let mut dial = init;
    let mut total_zeroes = 0;
    println!("{dial}({total_zeroes})");
    for line in input.lines() {
        match sign_rotation(line) {
            Err(e) => println!("{e}"),
            Ok(delta) => {
                let pretty = output(dial, delta);
                let zeroes;
                if part == 1 {
                    (dial, zeroes) = rotate(dial, delta);
                } else {
                    (dial, zeroes) = rotate_2(dial, delta);
                }
                total_zeroes += zeroes;
                println!("{pretty}={dial}    ({total_zeroes})");
            }
        }
    }
    Some(Number::Int32(total_zeroes))
}

fn rotate_2(dial: i32, delta: i32) -> (i32, i32) {
    let (applied, mut zeroes) = rotate(dial, delta);
    let mut extra_rotations = delta / 100;
    if extra_rotations < 0 {
        extra_rotations = -extra_rotations;
    }
    if extra_rotations > 0 {
        zeroes += extra_rotations;
    }
    if dial != 0 && applied != 0 && (delta < 0 && applied > dial || delta > 0 && applied < dial) {
        // if dial != 0 && applied != 0 && ((dial + delta) / 100 == 1 || dial + delta < 0) {
        zeroes += 1;
    }
    (applied, zeroes)
}

fn rotate(dial: i32, delta: i32) -> (i32, i32) {
    let mut normal_delta = delta % 100;
    let mut zero = 0;
    if normal_delta == 0 {
        return (dial, zero);
    } else if normal_delta < 0 {
        normal_delta += 100
    }
    let added_delta = dial + normal_delta;
    let applied = added_delta % 100;
    if applied == 0 {
        zero = 1
    }
    (applied, zero)
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
