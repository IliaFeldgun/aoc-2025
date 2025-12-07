use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;
use crate::mat_lib::mat::{echo_mat, square_submat, string_to_mat};

const DAY: i32 = 4;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    if part == 1 && let Some(rolls) = get_rolls(input, part) {
        println!("{rolls}");
        Some(Number::Int32(rolls))
    } else {
        None
    }
}

fn get_rolls(input: String, part: i32) -> Option<i32> {
    let mut mat = string_to_mat(&input);
    let matt = mat.clone();
    echo_mat(&mat);
    let mut rolls = 0;
    for (i, row) in mat.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == '@' {
                let sub_mat = square_submat(&matt, i, j, 3);
                println!("{i},{j}");
                echo_mat(&sub_mat);
                let mut sub_rolls = 0;
                for (si, sr) in sub_mat.iter().enumerate() {
                    for (sj, sc) in sr.iter().enumerate() {
                        if *sc == '@' || *sc == 'X' {
                            sub_rolls += 1;
                        }
                    }
                }
                if sub_rolls < 5 {
                    *cell = 'X';
                    println!("OK")
                } else {
                    println!("NOPE")
                }
            }
        }
    }
    echo_mat(&mat);
    for (i, row) in mat.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'X' {
                rolls += 1;
            }
        }
    }
    Some(rolls)
}
