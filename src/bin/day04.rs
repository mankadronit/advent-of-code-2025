use std::{fs::read_to_string, path::Path};

fn get_neighbors(row: usize, col: usize, max_rows: usize, max_cols: usize) -> Vec<(usize, usize)> {
    let deltas: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    deltas
        .iter()
        .filter_map(|(dr, dc)| {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            (nr >= 0 && nr < max_rows as i32 && nc >= 0 && nc < max_cols as i32)
                .then_some((nr as usize, nc as usize))
        })
        .collect()
}

fn pick_rolls(wall: &mut Vec<Vec<char>>) -> i32 {
    let mut rolls_picked = 0;

    for r in 0..wall.len() {
        for c in 0..wall[0].len() {
            let neighbors = get_neighbors(r, c, wall.len(), wall[0].len());
            let num_neighbors = neighbors
                .iter()
                .filter(|&&(nr, nc)| wall[nr][nc] == '@')
                .count();

            if wall[r][c] == '@' && num_neighbors < 4 {
                rolls_picked += 1;
                wall[r][c] = '.';
            }
        }
    }

    rolls_picked
}

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/inputs/day04_input.txt");

    let content = read_to_string(path)
        .expect("Error reading challenge input file. Please make sure it exists under the /inputs directory and is valid.");

    let mut wall: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut num_rolls = 0;

    loop {
        let picked = pick_rolls(&mut wall);
        if picked == 0 {
            break;
        }
        num_rolls += picked;
    }
    println!("{}", num_rolls)
}
