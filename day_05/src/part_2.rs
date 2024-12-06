use std::cmp::Ordering;

use crate::helpers::{is_correct_order, parse_input};

pub fn part_2(input: &str) -> i32 {
    let (order_map, mut updates) = parse_input(input);

    let mut valid = vec![];

    for update in &mut updates {
        if is_correct_order(&order_map, update) {
            continue;
        }

        update.sort_by(|a, b| {
            if let Some(a_requriments) = order_map.get(a) {
                if a_requriments.contains(b) {
                    return Ordering::Greater;
                }
            };

            if let Some(b_requriments) = order_map.get(b) {
                if b_requriments.contains(a) {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });

        let middle = update.len() / 2;
        let item = update[middle];
        valid.push(item);
    }

    valid.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 123);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
