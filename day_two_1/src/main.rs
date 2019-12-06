use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap_or_else(|_| "".to_owned());
    let mut numbers = input
        .split(',')
        .map(|x| x.parse().unwrap_or(99))
        .collect::<Vec<usize>>();
    intcode_program(&numbers);
}

fn intcode_program(codes: &[usize]) -> &[usize] {
    let mut cursor = 0;
    let mut ops = codes.get(cursor).unwrap_or(&99);
    let mut new_codes: Vec<usize> = Vec::with_capacity(codes.len());
    while let 1 | 2 = ops {
        let first_pos = codes.get(cursor + 1).unwrap_or(&0);
        let second_pos = codes.get(cursor + 2).unwrap_or(&0);
        let store_pos = codes.get(cursor + 3).unwrap_or(&0);
        let first_number = codes.get(*first_pos).unwrap();
        let second_number = codes.get(*second_pos).unwrap();
        let result = if ops == &1 {
            first_number + second_number
        } else {
            first_number * second_number
        };
        if let Some(elem) = new_codes.get_mut(*store_pos) {
            *elem = result
        }
        cursor += 4;
        ops = codes.get(cursor).unwrap_or(&99);
    }
    codes
}
