use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;

const DAY: i32 = 0;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    Some(Number::Int32(0))
}
