use crate::robot::{Quad, Robot};

pub fn part_1(input: &str, count: i32, dimensions: (i32, i32)) -> i32 {
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| Robot::new(line, dimensions))
        .collect();

    for mut robot in robots {
        for _ in 0..count {
            robot.step();
        }

        match robot.quad() {
            Some(Quad::TopLeft) => tl += 1,
            Some(Quad::TopRight) => tr += 1,
            Some(Quad::BottomLeft) => bl += 1,
            Some(Quad::BottomRight) => br += 1,
            None => {}
        }
    }

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
