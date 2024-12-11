use crate::helpers::{blink_n_times, parse_input};

pub fn part_1(input: &str) -> i64 {
    let input = parse_input(input);
    let final_stones = blink_n_times(input, 25);
    final_stones.len() as i64
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = "125 17";
        let output = part_1(input);
        assert_eq!(output, 55312);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
