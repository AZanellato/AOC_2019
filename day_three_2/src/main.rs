use std::collections::HashSet;
use std::fs;

type Movement = (i32, i32);
type Point = (i32, i32);

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| "".to_owned());

    let lines: Vec<HashSet<Point>> = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(line_to_positions)
        .map(movements_to_lines)
        .collect();

    let first_line = lines.get(0).unwrap();
    let second_line = lines.get(1).unwrap();
    let min_intersection = first_line
        .intersection(&second_line)
        .collect::<HashSet<_>>()
        .iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .unwrap_or(0);

    println!("{:?}", min_intersection);
}

fn line_to_positions(line: Vec<&str>) -> Vec<Movement> {
    line.into_iter().map(direction_to_movement).collect()
}

fn direction_to_movement(direction_magnitude: &str) -> Movement {
    let direction = direction_magnitude.chars().nth(0);
    let magnitude: i32 = direction_magnitude
        .get(1..)
        .map_or(0, |number| number.parse().unwrap_or(0));
    match direction {
        Some('R') => (magnitude, 0),
        Some('L') => (-magnitude, 0),
        Some('U') => (0, magnitude),
        Some('D') => (0, -magnitude),
        _ => (0, 0),
    }
}

fn movements_to_lines(movements: Vec<Movement>) -> HashSet<Point> {
    let mut current_location = (0, 0);
    let mut locations = HashSet::new();
    for movement in movements {
        if movement.0 != 0 {
            let mov = movement.0;
            let (range, amount) = if mov > 0 { (0..mov, 1) } else { (mov..0, -1) };
            for _ in range {
                current_location = (current_location.0 + amount, current_location.1);
                locations.insert(current_location);
            }
        } else {
            let mov = movement.1;
            let (range, amount) = if mov > 0 { (0..mov, 1) } else { (mov..0, -1) };
            for _ in range {
                current_location = (current_location.0, current_location.1 + amount);
                locations.insert(current_location);
            }
        }
    }
    locations
}
