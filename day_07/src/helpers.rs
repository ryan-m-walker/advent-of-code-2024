#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub value: u64,
    pub numbers: Vec<u64>,
}

pub fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(':').collect();
            let value: u64 = split[0].trim().parse().unwrap();
            let numbers = split[1]
                .trim()
                .split(' ')
                .map(|op| op.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            Equation { value, numbers }
        })
        .collect()
}

pub fn generate_permutations(count: i32) -> Vec<Vec<Op>> {
    let mut permutations = vec![];

    for i in 0..2_i32.pow(count as u32) {
        let mut permutation = vec![];

        for j in 0..count {
            let op = if i & (1 << j) == 0 {
                Op::Add
            } else {
                Op::Multiply
            };

            permutation.push(op);
        }

        permutations.push(permutation);
    }

    permutations
}

pub fn generate_permutations_with_concat(count: i32) -> Vec<Vec<Op>> {
    let mut permutations = vec![];

    for i in 0..3_i32.pow(count as u32) {
        let mut permutation = vec![];

        for j in 0..count {
            let op = match i / 3_i32.pow(j as u32) % 3 {
                0 => Op::Add,
                1 => Op::Multiply,
                2 => Op::Concat,
                _ => panic!("Invalid op"),
            };

            permutation.push(op);
        }

        permutations.push(permutation);
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let input = ["2: 1 1", "9: 3 3"].join("\n");
        let output = parse_input(&input);

        assert_eq!(
            output,
            vec![
                Equation {
                    value: 2,
                    numbers: vec![1, 1]
                },
                Equation {
                    value: 9,
                    numbers: vec![3, 3]
                }
            ]
        );
    }

    #[test]
    fn generate_permutations_test() {
        let output = generate_permutations(3);

        assert_eq!(
            output,
            vec![
                vec![Op::Add, Op::Add, Op::Add],
                vec![Op::Multiply, Op::Add, Op::Add],
                vec![Op::Add, Op::Multiply, Op::Add],
                vec![Op::Multiply, Op::Multiply, Op::Add],
                vec![Op::Add, Op::Add, Op::Multiply],
                vec![Op::Multiply, Op::Add, Op::Multiply],
                vec![Op::Add, Op::Multiply, Op::Multiply],
                vec![Op::Multiply, Op::Multiply, Op::Multiply]
            ]
        );
    }
}
