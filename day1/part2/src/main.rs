fn main() {
    let input = include_str!("./input.txt");

    let split: Vec<&str> = input.split_whitespace().collect();

    let left: Vec<i32> = split
        .iter()
        .step_by(2)
        .cloned()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let right: Vec<i32> = split
        .iter()
        .skip(1)
        .step_by(2)
        .cloned()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut sim_score = 0;

    for val in left {
        let occr = right.iter().filter(|&x| *x == val).count();
        sim_score += val * occr as i32;
    }

    println!("{sim_score}")
}
