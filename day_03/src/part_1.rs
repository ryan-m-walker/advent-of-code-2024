use crate::parser::{Node, Parser};

pub fn part_1(input: &str) -> i32 {
    let mut parser = Parser::new(input);
    let output = parser.parse();

    let mut sum = 0;

    for node in &output {
        if let Node::Mul(left, right) = node {
            let product = left * right;
            sum += product
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
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 161);
    }

    #[test]
    fn test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
