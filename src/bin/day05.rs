use std::path::Path;

use advent_of_code_2025::read_input;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day05_input.txt");

    let content = read_input(&path);
    let blank_line_idx = content
        .iter()
        .position(|line| line.is_empty())
        .expect("No Blank Line found");

    let mut fresh_ingredient_ranges: Vec<(i64, i64)> = (content[..blank_line_idx])
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Invalid range format");
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect();

    fresh_ingredient_ranges.sort_by_key(|&(start, _)| start);

    let mut ans = 0i64;
    let mut current_start = fresh_ingredient_ranges[0].0;
    let mut current_end = fresh_ingredient_ranges[0].1;

    for &(start, end) in &fresh_ingredient_ranges[1..] {
        if start > current_end {
            ans += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        } else {
            current_end = current_end.max(end);
        }
    }
    ans += current_end - current_start + 1;

    println!("Answer: {}", ans);
}
