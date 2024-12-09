use std::str::FromStr;

use regex::Regex;

fn multiply(mul: &str) -> i32 {
    let mut op = String::from_str(mul).unwrap();
    op = op.strip_prefix("mul(").unwrap().to_string();
    op = op.strip_suffix(")").unwrap().to_string();
    let mut val_iter = op.split(",");
    let x: i32 = val_iter.next().unwrap().parse().unwrap();
    let y: i32 = val_iter.next().unwrap().parse().unwrap();
    return x * y;
}

fn main() {
    let input = include_str!("./input.txt");

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)").unwrap();

    let mut total = 0;
    let mut enabled = true;

    for m in re.find_iter(input) {
        let op = m.as_str();

        if enabled && op == "don't()" {
            enabled = false;
            continue;
        }

        if !enabled && op == "do()" {
            enabled = true;
            continue;
        }

        if enabled && op.starts_with("mul") {
            total += multiply(op);
        }
    }

    println!("{total}")
}
