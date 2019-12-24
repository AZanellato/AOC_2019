use std::collections::HashMap;

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
    let d0 = ((number / 100_000) % 10) as u8;
    let d1 = ((number / 10_000) % 10) as u8;
    let d2 = ((number / 1_000) % 10) as u8;
    let d3 = ((number / 100) % 10) as u8;
    let d4 = ((number / 10) % 10) as u8;
    let d5 = (number % 10) as u8;
    let digits = [d0, d1, d2, d3, d4, d5];
    let mut sorted_digits = [d0, d1, d2, d3, d4, d5];
    sorted_digits.sort_by(|a, b| a.cmp(b));

    digits == sorted_digits
}

fn check_for_double(number: u32) -> bool {
    let d0 = ((number / 100_000) % 10) as u8;
    let d1 = ((number / 10_000) % 10) as u8;
    let d2 = ((number / 1_000) % 10) as u8;
    let d3 = ((number / 100) % 10) as u8;
    let d4 = ((number / 10) % 10) as u8;
    let d5 = (number % 10) as u8;
    let digits = [d0, d1, d2, d3, d4, d5];
    let mut digit_counts = HashMap::new();
    for digit in digits.iter() {
        let count = digit_counts.entry(digit).or_insert(0);
        *count += 1;
    }
    digit_counts.values().any(|&value| value == 2)
}

#[cfg(test)]
mod tests {
    use super::{check_for_double, check_not_decreasing};

    #[test]
    fn double_test_cases() {
        assert_eq!(check_for_double(111_222), false);
        assert_eq!(check_for_double(111_111), false);
        // assert_eq!(check_for_double(123444), false);
        // assert_eq!(check_for_double(112233), true);
        assert_eq!(check_for_double(111_255), true);
        // assert_eq!(check_for_double(111122), true);
        // assert_eq!(check_for_double(111223788), true);
        // assert_eq!(check_for_double(111223788), true);
    }

    #[test]
    fn all_true_cases() {
        assert_eq!(check_for_double(445_550), true);
        assert_eq!(check_for_double(446660), true);
        assert_eq!(check_for_double(447770), true);
        assert_eq!(check_for_double(448880), true);
        assert_eq!(check_for_double(449990), true);
        assert_eq!(check_for_double(556660), true);
        assert_eq!(check_for_double(557770), true);
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
