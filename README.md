# Advent of Code 2025

Solutions to [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Structure

```
src/
  lib.rs              # Shared utilities (input reading)
  bin/
    day01.rs          # Day 1 solution
    inputs/
      day01_input.txt # Puzzle inputs (not committed)
```

Each day is a standalone binary under `src/bin/`. Puzzle inputs live in `src/bin/inputs/` and are gitignored.

## Running

```bash
# Run a specific day
cargo run --bin day01

# Run all days
cargo run --bins
```

## Adding a New Day

1. Add `src/bin/dayNN.rs` with a `main` function
2. Add your puzzle input to `src/bin/inputs/dayNN_input.txt`
3. Use `read_input` from the crate root to load it:

```rust
use advent_of_code_2025::read_input;

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/bin/inputs/dayNN_input.txt");
    let content = read_input(&path);
    // ...
}
```
