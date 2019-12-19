use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| "".to_owned());

    let mapped_lines = input.lines().map(|line| {
        let mass: i32 = line.parse().unwrap();
        mass / 3 - 2
    });
}
