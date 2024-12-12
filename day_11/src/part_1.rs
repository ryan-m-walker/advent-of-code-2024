use crate::stones::Stones;

pub fn part_1(input: &str, n: i32) -> i64 {
    let mut stones = Stones::new(input);
    stones.blink_n_times(n)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = "125 17";
        let output = part_1(input, 6);
        assert_eq!(output, 22);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input, 25);
        dbg!(output);
    }
}
