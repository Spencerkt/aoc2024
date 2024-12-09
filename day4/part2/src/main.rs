fn xmas(mini_matrix: Vec<Vec<String>>) -> bool {
    let diag_x = [
        mini_matrix[0][0].clone(),
        mini_matrix[1][1].clone(),
        mini_matrix[2][2].clone(),
    ]
    .join("");

    if diag_x != "MAS" && diag_x != "SAM" {
        return false;
    }

    let diag_y = [
        mini_matrix[0][2].clone(),
        mini_matrix[1][1].clone(),
        mini_matrix[2][0].clone(),
    ]
    .join("");

    return diag_y == "MAS" || diag_y == "SAM";
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

    let mut xmas_count = 0;

    for row_idx in 0..rows {
        for col_idx in 0..cols {
            if row_idx + 3 > rows {
                break;
            }
            if col_idx + 3 > cols {
                break;
            }

            let window = vec![
                matrix[row_idx][col_idx..col_idx + 3].to_vec(),
                matrix[row_idx + 1][col_idx..col_idx + 3].to_vec(),
                matrix[row_idx + 2][col_idx..col_idx + 3].to_vec(),
            ];

            if xmas(window) {
                xmas_count += 1;
            }
        }
    }

    println!("xmas count: {xmas_count}");
}
