use crate::helpers::{is_safe, parse_input};

pub fn part_1(input: &str) -> i32 {
    let data = parse_input(input);
    let mut output = 0;

    for line in data {
        if is_safe(&line) {
            output += 1;
            continue;
        }

        for index in 0..line.len() {
            let mut filtered = line.clone();
            filtered.remove(index);

            if is_safe(&filtered) {
                output += 1;
                break;
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
