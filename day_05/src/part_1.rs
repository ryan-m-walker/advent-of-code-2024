use std::{collections::HashSet, hash::RandomState};

use crate::helpers::parse_input;

pub fn part_1(input: &str) -> i32 {
    let (order_map, updates) = parse_input(input);

    let mut valid = vec![];

    'updates: for update in updates {
        let all: HashSet<&i32, RandomState> = HashSet::from_iter(update.iter());
        let mut seen = HashSet::new();

        'page: for page in &update {
            seen.insert(page);

            let Some(requirments) = order_map.get(page) else {
                continue;
            };

            for requirment in requirments {
                if !all.contains(requirment) {
                    continue;
                }

                if !seen.contains(requirment) {
                    continue 'updates;
                }
            }
        }

        let middle = seen.len() / 2;

        let Some(item) = update.get(middle) else {
            continue;
        };

        valid.push(*item);
    }

    valid.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 143);
    }
    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
