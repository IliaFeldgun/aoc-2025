use crate::aoc_lib::expect::Number;
use crate::aoc_lib::main as aoc_main;
use crate::mat_lib::mat::{echo_mat, square_submat, string_to_mat};

const DAY: i32 = 4;

pub fn main() {
    aoc_main::main(DAY, run_part);
}

fn run_part(part: i32, input: String) -> Option<Number> {
    if let Some(rolls) = get_rolls(input, part) {
        println!("{rolls}");
        Some(Number::Int32(rolls))
    } else {
        None
    }
}

fn get_rolls(input: String, part: i32) -> Option<i32> {
    let mut mat = string_to_mat(&input);
    echo_mat(&mat);
    let mut rolls = -1;
    let mut new_rolls = 0;
    while new_rolls != rolls {
        let matt = mat.clone();
        rolls = new_rolls;
        for (i, row) in mat.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if *cell == '@' {
                    let sub_mat = square_submat(&matt, i, j, 3);
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
                    }
                }
            }
        }
        for (i, row) in mat.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if *cell == 'X' {
                    *cell = 'x';
                    new_rolls += 1;
                }
            }
        }
        if part == 1 {
            return Some(new_rolls);
        }
    }
    Some(rolls)
}
