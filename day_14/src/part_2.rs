use crate::robots::Robots;

pub fn part_2(input: &str, dimensions: (i32, i32)) -> i32 {
    let mut steps = 0;
    let mut robots = Robots::new(input, dimensions);

    loop {
        if robots.maybe_tree() {
            println!();
            robots.print();
            println!();
            return steps;
        }

        steps += 1;
        robots.step();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input, (101, 103));
        dbg!(output);
    }
}
