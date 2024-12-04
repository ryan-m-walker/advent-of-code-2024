use crate::helpers::parse_input;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Asc,
    Desc,
}

pub fn part_1(input: &str) -> i32 {
    let data = parse_input(input);
    let mut output = 0;

    'outer: for line in data {
        let mut direction: Option<Direction> = None;

        for tup in line.windows(2) {
            let current = tup[0];
            let next = tup[1];

            let diff = current - next;

            let new_direction = if diff > 0 {
                Direction::Desc
            } else {
                Direction::Asc
            };

            if let Some(direction) = direction {
                if direction != new_direction {
                    continue 'outer;
                }
            }

            if direction.is_none() {
                direction = Some(new_direction);
            }

            if !(1..=3).contains(&diff.abs()) {
                continue 'outer;
            }
        }

        output += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 2);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
