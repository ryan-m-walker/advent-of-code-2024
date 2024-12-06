use crate::finder::Finder;

pub fn part_2(input: &str) -> i32 {
    let mut finder = Finder::new(input);
    finder.find_mas_x_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
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
