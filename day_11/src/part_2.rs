use crate::helpers::{blink_n_times, parse_input};

pub fn part_2(input: &str) -> i64 {
    let input = parse_input(input);
    let final_stones = blink_n_times(input, 75);
    final_stones.len() as i64
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    // #[test]
    // fn part_2_input() {
    //     let input = fs::read_to_string("./input.txt").unwrap();
    //     let output = part_2(&input);
    //     dbg!(output);
    // }
}
