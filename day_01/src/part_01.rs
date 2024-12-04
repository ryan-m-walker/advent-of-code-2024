use std::iter::zip;

use crate::helpers::parse_input;

pub fn part_01(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| (r - l).abs()).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./test_input.txt").expect("Failed to read input file");
        let output = part_01(&input);
        assert_eq!(output, 11);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
        let output = part_01(&input);
        dbg!(output);
    }
}
