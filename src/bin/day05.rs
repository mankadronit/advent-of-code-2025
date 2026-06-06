use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::path::Path;

use advent_of_code_2025::read_input;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day05_input.txt");

    let content = read_input(&path);
    let blank_line_idx = content
        .iter()
        .position(|line| line.is_empty())
        .expect("No Blank Line found");

    let fresh_ingredient_ranges: &Vec<RangeInclusive<i64>> = &(content[..blank_line_idx])
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Invalid range format");
            start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap()
        })
        .collect();

    let ans = content[(blank_line_idx + 1)..]
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect::<HashSet<i64>>()
        .iter()
        .filter(|i| fresh_ingredient_ranges.iter().any(|r| r.contains(i)))
        .count();

    println!("Answer: {ans}");
}
