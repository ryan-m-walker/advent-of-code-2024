use crate::parser::{Node, Parser};

pub fn part_2(input: &str) -> i32 {
    let mut parser = Parser::new(input);
    let output = parser.parse();

    let mut sum = 0;
    let mut should_run = true;

    for node in &output {
        match node {
            Node::Do => should_run = true,
            Node::Dont => should_run = false,
            Node::Mul(left, right) => {
                if should_run {
                    let product = left * right;
                    sum += product
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_test() {
        let input = fs::read_to_string("./example_input_part_2.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 48);
    }

    #[test]
    fn test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
