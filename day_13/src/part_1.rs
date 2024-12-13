use crate::helpers::parse_input;

pub fn part_1(input: &str) -> i32 {
    let machines = parse_input(input);
    let mut total_cost = 0;

    for machine in machines {
        if let Some((a, b)) = machine.brute_force() {
            let a_cost = a * 3;
            total_cost += a_cost + b;
        }
    }

    total_cost
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
