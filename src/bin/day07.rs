use advent_of_code_2025::read_input;
use std::{collections::HashSet, path::Path};

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day07_input.txt");

    let content: Vec<Vec<i32>> = read_input(&path)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '^' || c == 'S' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut entry_points: HashSet<usize> = content[0]
        .iter()
        .enumerate()
        .filter(|&(_, v)| *v == 1)
        .map(|(i, _)| i)
        .collect();

    let mut ans = 0;

    for row in &content[1..] {
        let row_indices: Vec<usize> = row
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 1)
            .map(|(i, _)| i)
            .collect();

        let overlaps: Vec<usize> = entry_points
            .iter()
            .filter(|i| row_indices.contains(i))
            .copied()
            .collect();

        ans += overlaps.len();

        for i in overlaps {
            if i > 0 {
                entry_points.insert(i - 1);
            }

            if i < content[0].len() {
                entry_points.insert(i + 1);
            }
            entry_points.remove(&i);
        }
    }

    println!("Answer: {}", ans);
}
