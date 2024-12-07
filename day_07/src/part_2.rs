use crate::helpers::{generate_permutations_with_concat, parse_input, Op};

pub fn part_2(input: &str) -> u64 {
    let equations = parse_input(input);

    let mut sum = 0;

    // not that slow, guess it works? ¯\_(ツ)_/¯
    'outer: for equation in equations {
        let permutations = generate_permutations_with_concat(equation.numbers.len() as i32 - 1);

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
                    Op::Concat => {
                        result = format!("{}{}", result, next_num).parse().unwrap();
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
    fn part_2_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 11387);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
