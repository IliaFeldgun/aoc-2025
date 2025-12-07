pub fn string_to_mat(input: &str) -> Vec<Vec<char>> {
    let mut rows = 0;
    let mut cols = 0;
    for line in input.lines() {
        if line.len() > cols {
            cols = line.len();
        }
        rows += 1;
    }
    let mut char_matrix: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
    for (i, line) in input.lines().enumerate() {
        for (j, value) in line.chars().enumerate() {
            char_matrix[i][j] = value;
        }
    }
    char_matrix
}

pub fn square_submat(mat: &[Vec<char>], row: usize, col: usize, size: usize) -> Vec<Vec<char>> {
    let mut sub_matrix: Vec<Vec<char>> = vec![vec![' '; size]; size];
    let corner_row = row.saturating_sub(size / 2);
    let corner_col = col.saturating_sub(size / 2);
    for i in 0..size {
        for j in 0..size {
            let out_of_bounds_row = row as i32 - size as i32 / 2 + (i as i32);
            let out_of_bounds_col = col as i32 - size as i32 / 2 + (j as i32);
            if out_of_bounds_row < corner_row as i32 {
                sub_matrix[i][j] = '_';
            } else if out_of_bounds_col < corner_col as i32 {
                sub_matrix[i][j] = '>';
            } else if let Some(line) = mat.get(out_of_bounds_row as usize) {
                if let Some(cell) = line.get(out_of_bounds_col as usize) {
                    sub_matrix[i][j] = *cell
                } else {
                    sub_matrix[i][j] = '<';
                }
            } else {
                sub_matrix[i][j] = '^';
            }
        }
    }
    sub_matrix
}

pub fn echo_mat(mat: &Vec<Vec<char>>) {
    let mut output = String::new();
    for row in mat {
        for cell in row {
            output = format!("{output}{cell}")
        }
        output = format!("{output}\n")
    }
    println!("{output}")
}
