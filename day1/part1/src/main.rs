use core::str;
use std::iter::zip;

fn main() {
    //let input = include_str!("./input-mini.txt");
    let input = include_str!("./input.txt");

    let split: Vec<&str> = input.split_whitespace().collect();

    let mut left: Vec<i32> = split
        .iter()
        .step_by(2)
        .cloned()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut right: Vec<i32> = split
        .iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    left.sort();
    right.sort();

    let sum: i32 = zip(left.clone().into_iter(), right.clone().into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("{sum}")
}
