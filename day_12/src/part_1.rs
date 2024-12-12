use crate::farm::Farm;

pub fn part_1(input: &str) -> i32 {
    let farm = Farm::new(input);
    let regions = farm.get_regions();

    let mut output = 0;

    for region in regions {
        output += region.parimeter * region.area;
    }

    output
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 772);
    }

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 1930);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
