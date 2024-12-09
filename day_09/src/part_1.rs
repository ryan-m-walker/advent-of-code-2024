use std::mem;

use crate::helpers::{format_disk, parse_input, Block};

pub fn part_1(input: &str) -> i64 {
    let mut blocks = parse_input(input);
    format_disk(&mut blocks);

    let mut checksum: i64 = 0;

    for (i, block) in blocks.iter().enumerate() {
        let Block::File(id) = block else {
            continue;
        };

        checksum += id * i as i64;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 1928);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
