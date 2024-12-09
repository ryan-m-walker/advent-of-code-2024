use crate::{
    helpers::{parse_input, Block},
    memory::Memory,
};

pub fn part_2(input: &str) -> i64 {
    let mut blocks = parse_input(input);
    let mut memory = Memory::new(&mut blocks);
    memory.compress();

    let mut checksum: i64 = 0;

    for (i, block) in memory.blocks.iter().enumerate() {
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
    fn part_2_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 2858);
    }

    #[test]
    fn part_2_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
