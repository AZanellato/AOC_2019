use std::collections::HashMap;
use std::iter::{FromIterator, Iterator};
fn main() {
    let lower_bound = 147_981;
    let upper_bound = 691_423;

    let count = (lower_bound..upper_bound)
        .filter(|n| check_not_decreasing(*n))
        .filter(|n| check_for_double(*n))
        .count();

    println!("The count is: {}", count);
}

fn check_not_decreasing(number: u32) -> bool {
    let mut digits = number.to_string().chars().collect::<Vec<char>>();
    digits.sort_by(|a, b| a.cmp(b));

    let digits = String::from_iter(digits);
    number.to_string() == digits
}

fn check_for_double(number: u32) -> bool {
    let string_digits = number.to_string();
    let mut digits = string_digits.chars().peekable();
    let current_double: Option<char> = None;
    let mut result = false;
    while let Some(dig) = digits.next() {
        if dig == *digits.peek().unwrap_or_else(|| &' ') {
            result = true;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{check_for_double, check_not_decreasing};

    #[test]
    fn double_test_cases() {
        assert_eq!(check_for_double(1222), false);
        assert_eq!(check_for_double(155), true);
        assert_eq!(check_for_double(111111), false);
        assert_eq!(check_for_double(11111122), true);
    }

    #[test]
    fn decreasing_test_cases() {
        assert_eq!(check_not_decreasing(12), true);
        assert_eq!(check_not_decreasing(155), true);
        assert_eq!(check_not_decreasing(111111), true);
        assert_eq!(check_not_decreasing(111101), false);
        assert_eq!(check_not_decreasing(223450), false);
    }
}
