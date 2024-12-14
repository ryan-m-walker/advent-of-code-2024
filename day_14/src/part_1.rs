use crate::{
    robot::{Quad, Robot},
    robots::Robots,
};

pub fn part_1(input: &str, count: i32, dimensions: (i32, i32)) -> i32 {
    let mut robots = Robots::new(input, dimensions);
    robots.step_n_times(count);
    let [tl, tr, bl, br] = robots.quads();
    tl * tr * bl * br
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input, 100, (11, 7));
        assert_eq!(output, 12);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input, 100, (101, 103));
        dbg!(output);
    }
}
