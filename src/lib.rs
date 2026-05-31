use std::fs;
use std::path::Path;

pub fn read_input(path: &Path) -> Vec<String> {
    let content = fs::read_to_string(path)
        .expect("Error reading challenge input file. Please make sure it exists under the /inputs directory and is valid.");

    content.lines().map(String::from).collect()
}
