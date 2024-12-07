use core::str;
use std::i32;

fn safe(levels: Vec<i32>) -> bool {
    let increasing = levels[0] < levels[1];
    let mut is_safe = true;

    for i in 1..levels.len() {
        // Check delta between any 2 levels between [1, 3]
        let diff = levels[i - 1] - levels[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            is_safe = false;
            break;
        }
        // If increasing, i-1 must be less than i
        if increasing && levels[i - 1] > levels[i] {
            is_safe = false;
            break;
        }
        // If decreasing, i-1 must be more than i
        if !increasing && levels[i - 1] < levels[i] {
            is_safe = false;
            break;
        }
    }

    return is_safe;
}

fn main() {
    let reports: Vec<&str> = include_str!("./input.txt")
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    let mut safe_count = 0;

    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if safe(levels) {
            safe_count += 1;
        }
    }

    println!("{safe_count}")
}
