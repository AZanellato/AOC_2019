use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| "".to_owned());
    println!("{}", input);
    let numbers = input
        .split(',')
        .map(|x| x.parse().unwrap_or_else(|_| 0))
        .collect::<Vec<usize>>();
    println!("{:?}", numbers);
    let new_numbers = intcode_program(&numbers);

    println!("{:?}", new_numbers);
}

fn intcode_program(codes: &[usize]) -> Vec<usize> {
    let mut new_codes = codes.to_vec();
    let mut cursor = 0;
    let mut ops = new_codes.get(cursor).unwrap();
    while let 1 | 2 = ops {
        let first_pos = new_codes.get(cursor + 1).unwrap();
        let second_pos = new_codes.get(cursor + 2).unwrap();
        let store_pos = *new_codes.get(cursor + 3).unwrap();
        let first_number = new_codes.get(*first_pos).unwrap();
        let second_number = new_codes.get(*second_pos).unwrap();
        println!(
            "Operation: {}, with numbers {} and {} from position {} and {}, store that on store_pos {}",
            ops, first_number, second_number, first_pos, second_pos, store_pos
        );
        let result = if *ops == 1 {
            first_number + second_number
        } else if *ops == 2 {
            first_number * second_number
        } else {
            0
        };
        if let Some(elem) = new_codes.get_mut(store_pos) {
            *elem = result;
        }
        cursor += 4;
        println!("{:?}", new_codes);
        ops = new_codes.get(cursor).unwrap();
    }

    new_codes
}

#[cfg(test)]
mod tests {
    use super::intcode_program;

    #[test]
    fn example_test_cases() {
        assert_eq!(intcode_program(&[1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
        assert_eq!(intcode_program(&[2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
        assert_eq!(
            intcode_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(
            intcode_program(&[2, 4, 4, 5, 99, 0]),
            [2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            intcode_program(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
