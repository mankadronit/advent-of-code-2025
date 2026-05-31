use advent_of_code_2025::read_input;

fn is_mirrored(n: i64) -> bool {
    let s = n.to_string();
    let mid = s.len() / 2;
    s[..mid] == s[mid..]
}

fn main() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day02_input.txt");

    let content = read_input(&path);

    let answer: i64 = content
        .iter()
        .flat_map(|line| line.split(','))
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            start..=end
        })
        .filter(|&n| is_mirrored(n))
        .sum();

    println!("Answer: {}", answer);
}
