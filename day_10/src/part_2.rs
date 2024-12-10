use crate::map::HikingMap;

pub fn part_2(input: &str) -> i32 {
    let map = HikingMap::new(input);
    map.find_all_unique_trailheads()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_2_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 81);
    }

    #[test]
    fn part_2_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
