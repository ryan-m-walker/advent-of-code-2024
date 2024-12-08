use crate::map::Map;

pub fn part_2(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.get_all_antinodes()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_2_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 34);
    }

    #[test]
    fn part_2_example_2_test() {
        let input = fs::read_to_string("./example_input_2.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 9);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
