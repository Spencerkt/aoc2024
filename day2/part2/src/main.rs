use core::str;
use std::i32;

fn safe(levels: Vec<i32>) -> bool {
    let increasing = levels[0] < levels[levels.len() - 1];
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

        // Safe without dampeneing?
        if safe(levels.clone()) {
            safe_count += 1;
            continue;
        }

        // Brute force attempt to dampen.
        // Lots of edgecases around which level is the "bad" level.
        let mut safe_with_dampening = false;
        for i in 0..levels.len() {
            let mut with_dampeneing = levels.clone();
            with_dampeneing.remove(i);

            if safe(with_dampeneing) {
                safe_with_dampening = true;
                break;
            }
        }

        if safe_with_dampening {
            safe_count += 1;
        } else {
            // just debugging
            //println!("unsafe: {:?}", levels);
        }
    }

    println!("{safe_count}")
}
