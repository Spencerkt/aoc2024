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

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut total = 0;

    for x in re.captures_iter(input) {
        let (mul, []) = x.extract();
        total += multiply(mul);
    }

    println!("{total}")
}
