use aoc::five::main as main5;
use aoc::four::main as main4;
use aoc::one::main as main1;
use aoc::three::main as main3;
use aoc::two::main as main2;

mod aoc;
mod aoc_lib;
mod mat_lib;

fn main() {
    println!("AOC 2025!");
    main1();
    main2();
    main3();
    main4();
    main5();
}
