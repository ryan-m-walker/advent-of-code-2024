use crate::{guard::Guard, map::Map};

pub fn part_1(input: &str) -> i32 {
    let map = Map::new(input);
    let mut guard = Guard::new(&map);
    guard.patrol()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 41);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
