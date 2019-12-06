use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap_or("0".to_owned());
    let answer = calculate_total_fuel(input);
    println!("Fuel needed: {}", answer);
}

fn calculate_total_fuel(input: String) -> i32 {
    input
        .lines()
        .map(|line| calculate_fuel(line.parse().unwrap()))
        .sum()
}

fn calculate_fuel(initial_mass: i32) -> i32 {
    FuelIterator { mass: initial_mass }.sum()
}

struct FuelIterator {
    mass: i32,
}

impl Iterator for FuelIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.mass = self.mass / 3 - 2;
        if self.mass > 0 {
            Some(self.mass)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_fuel;

    #[test]
    fn example_test_cases() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
