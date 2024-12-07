use crate::helpers::{generate_permutations, parse_input, Op};

pub fn part_1(input: &str) -> u64 {
    let equations = parse_input(input);

    let mut sum = 0;

    // oh boy, brute forcing it. i'm sure part 2 won't make me regret this solution!
    'outer: for equation in equations {
        let permutations = generate_permutations(equation.numbers.len() as i32 - 1);

        for permutation in permutations {
            let mut result = equation.numbers[0];

            for (i, op) in permutation.iter().enumerate() {
                let next_num = equation.numbers[i + 1];

                match op {
                    Op::Add => {
                        result += next_num;
                    }
                    Op::Multiply => {
                        result *= next_num;
                    }
                }
            }

            if result == equation.value {
                sum += equation.value;
                continue 'outer;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 3749);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
