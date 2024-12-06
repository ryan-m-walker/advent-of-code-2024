use crate::helpers::{is_correct_order, parse_input};

pub fn part_2(input: &str) -> i32 {
    let (order_map, updates) = parse_input(input);

    for update in &updates {
        if is_correct_order(&order_map, update) {
            continue;
        }

        for page in update {
            // while !is_correct_order(&order_map, update) {
            //     //
            // }
        }
    }

    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_example_test() {
        // let input = fs::read_to_string("./example_input.txt").unwrap();
        // let _output = part_2(&input);
    }
}
