use crate::helpers::{is_correct_order, parse_input};

pub fn part_1(input: &str) -> i32 {
    let (order_map, updates) = parse_input(input);

    let mut valid = vec![];

    for update in updates {
        if is_correct_order(&order_map, &update) {
            let middle = update.len() / 2;
            let item = update[middle];
            valid.push(item);
        }
    }

    valid.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 143);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
