use std::path::Path;

use advent_of_code_2025::read_input;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day06_input.txt");
    let content: Vec<Vec<String>> = read_input(&path)
        .into_iter()
        .map(|line| line.split_whitespace().map(String::from).collect())
        .collect();

    let num_problems = content[0].len();
    let mut problems: Vec<Vec<String>> = vec![vec![]; num_problems];

    for row in &content {
        for (i, val) in row.iter().enumerate() {
            problems[i].push(val.to_string());
        }
    }

    let mut answer = 0i64;
    for problem in problems {
        let op: &str = problem.last().unwrap();

        match op {
            "*" => {
                answer += problem[0..(problem.len() - 1)]
                    .iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .product::<i64>()
            }
            "+" => {
                answer += problem[0..(problem.len() - 1)]
                    .iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .sum::<i64>()
            }
            &_ => println!("Unknown operator"),
        }
    }

    println!("{}", answer)
}
