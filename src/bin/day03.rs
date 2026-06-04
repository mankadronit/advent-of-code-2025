use advent_of_code_2025::read_input;
use std::path::Path;

fn calculate_joltage(bank: &str) -> i64 {
    let mut discard_budget = bank.len() - 12;
    let mut stack: Vec<i64> = Vec::new();

    for digit in bank.chars().map(|c| c.to_digit(10).unwrap() as i64) {
        if !stack.is_empty() {
            while discard_budget > 0 && !stack.is_empty() && stack.last().unwrap() < &digit {
                discard_budget -= 1;
                stack.pop();
            }
        }
        stack.push(digit);
    }

    stack.truncate(12);

    let joltage = stack
        .iter()
        .enumerate()
        .map(|(index, b)| 10i64.pow(11 - (index as u32)) * b)
        .sum();

    joltage
}

fn main() {
    let path = Path::new("src/bin/inputs/day03_input.txt");
    let content = read_input(path);

    let answer: i64 = content.iter().map(|bank| calculate_joltage(bank)).sum();

    println!("Answer is: {answer}");
}
