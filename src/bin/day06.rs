use std::path::Path;

use advent_of_code_2025::read_input;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day06_input.txt");
    let lines = read_input(&path);

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Transpose: cols[c] = all characters in column c, top to bottom
    let cols: Vec<Vec<char>> = (0..max_len)
        .map(|c| {
            lines
                .iter()
                .map(|line| line.chars().nth(c).unwrap_or(' '))
                .collect()
        })
        .collect();

    let grand_total: i64 = cols
        .split(|col| col.iter().all(|&c| c == ' '))
        .filter(|problem| !problem.is_empty())
        .map(|problem| {
            let op = problem
                .iter()
                .find_map(|col| col.last().copied().filter(|&c| c != ' '))
                .unwrap();

            let numbers: Vec<i64> = problem
                .iter()
                .rev()
                .filter_map(|col| {
                    let digits: String =
                        col[..col.len() - 1].iter().filter(|&&c| c != ' ').collect();
                    digits.parse().ok()
                })
                .collect();

            match op {
                '*' => numbers.iter().product::<i64>(),
                '+' => numbers.iter().sum::<i64>(),
                _ => panic!("Unknown operator: {}", op),
            }
        })
        .sum();

    println!("Part 2: {}", grand_total);
}
