use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap_or("0".to_owned());
    let answer = calculate_fuel(input);
    println!("Fuel needed: {}", answer);
}

fn calculate_fuel(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mass: i32 = line.parse().unwrap();
            mass / 3 - 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::calculate_fuel;

    #[test]
    fn example_test_cases() {
        assert_eq!(calculate_fuel("12".to_owned()), 2);
        assert_eq!(calculate_fuel("14".to_owned()), 2);
        assert_eq!(calculate_fuel("1969".to_owned()), 654);
        assert_eq!(calculate_fuel("100756".to_owned()), 33583);
    }
}
