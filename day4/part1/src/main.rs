use std::i32;

fn count(s: String) -> i32 {
    if s.len() < 4 {
        return 0;
    }

    let xmas_count: i32 = s
        .as_bytes()
        .windows(4)
        .filter(|&w| w == String::from("XMAS").as_bytes())
        .count()
        .try_into()
        .unwrap();

    let samx_count: i32 = s
        .as_bytes()
        .windows(4)
        .filter(|&w| w == String::from("SAMX").as_bytes())
        .count()
        .try_into()
        .unwrap();

    return xmas_count + samx_count;
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{input}");

    let matrix: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    println!("rows {rows}");
    println!("cols {cols}");

    let mut lines: Vec<String> = vec![];

    // Collect rows
    for row_idx in 0..rows {
        let mut col_vec: Vec<String> = [].to_vec();

        for col_idx in 0..cols {
            col_vec.push(matrix[row_idx][col_idx].clone());
        }

        let col = col_vec.join("");
        lines.push(col);
    }

    // Collect cols
    for row_idx in 0..rows {
        let mut col_vec: Vec<String> = [].to_vec();

        for col_idx in 0..cols {
            col_vec.push(matrix[col_idx][row_idx].clone());
        }

        let col = col_vec.join("");
        lines.push(col);
    }

    // Collect top-left to bottom-right diagonals
    for start_row in 0..rows {
        let mut diagonal = vec![];
        let mut r = start_row;
        let mut c = 0;
        while r < rows && c < cols {
            diagonal.push(matrix[r][c].clone());
            r += 1;
            c += 1;
        }
        lines.push(diagonal.join(""));
    }

    for start_col in 1..cols {
        let mut diagonal = vec![];
        let mut r = 0;
        let mut c = start_col;
        while r < rows && c < cols {
            diagonal.push(matrix[r][c].clone());
            r += 1;
            c += 1;
        }
        lines.push(diagonal.join(""));
    }

    // Collect top-right to bottom-left diagonals
    for start_row in 0..rows {
        let mut diagonal = vec![];
        let mut r = start_row;
        let mut c = cols as isize - 1;
        while r < rows && c >= 0 {
            diagonal.push(matrix[r][c as usize].clone());
            r += 1;
            c -= 1;
        }
        lines.push(diagonal.join(""));
    }

    for start_col in (0..cols - 1).rev() {
        let mut diagonal = vec![];
        let mut r = 0;
        let mut c = start_col as isize;
        while r < rows && c >= 0 {
            diagonal.push(matrix[r][c as usize].clone());
            r += 1;
            c -= 1;
        }
        lines.push(diagonal.join(""));
    }

    let found: i32 = lines.iter().map(|l| count(l.to_string())).sum();

    println!("found: {found}");
}
