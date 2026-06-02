use advent_of_code_2025::read_input;
use std::{cmp::max, path::Path};

fn calculate_joltage(bank: &str) -> i32 {
    let batteries: Vec<i32> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut max_joltage = 0;

    for i in 0..batteries.len() {
        for j in 0..i {
            max_joltage = max(max_joltage, batteries[j] * 10 + batteries[i]);
        }
    }

    max_joltage
}

fn main() {
    let path = Path::new("src/bin/inputs/day03_input.txt");
    let content = read_input(path);

    let answer: i32 = content.iter().map(|bank| calculate_joltage(bank)).sum();

    println!("Answer is: {answer}");
}
