use advent_of_code_2025::read_input;
use std::{collections::HashMap, path::Path};

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day07_input.txt");

    let content: Vec<Vec<i64>> = read_input(&path)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '^' || c == 'S' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut timelines: HashMap<usize, i64> = content[0]
        .iter()
        .enumerate()
        .filter(|&(_, v)| *v == 1)
        .map(|(i, _)| (i, 1i64))
        .collect();

    for row in &content[1..] {
        let row_indices: Vec<usize> = row
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 1)
            .map(|(i, _)| i)
            .collect();

        let overlaps: Vec<(usize, i64)> = timelines
            .iter()
            .filter(|(i, _)| row_indices.contains(i))
            .map(|(&i, &count)| (i, count))
            .collect();

        for (i, count) in overlaps {
            if i > 0 {
                *timelines.entry(i - 1).or_insert(0) += count;
            }

            if i < content[0].len() {
                *timelines.entry(i + 1).or_insert(0) += count;
            }

            timelines.remove(&i);
        }
    }

    let ans: i64 = timelines.values().sum();
    println!("Answer: {}", ans);
}
