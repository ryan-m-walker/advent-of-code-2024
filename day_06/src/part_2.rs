use crate::{guard::Guard, map::Map};

pub fn part_2(input: &str) -> i32 {
    let map = Map::new(input, None);
    let mut guard = Guard::new(&map);

    let visited = guard.patrol().unwrap();

    let mut loops = 0;

    for visit in visited {
        let map = Map::new(input, Some(visit));
        let mut guard = Guard::new(&map).with_print_map(false);

        if guard.patrol().is_none() {
            loops += 1;
        }
    }

    loops
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 6);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
