use std::collections::HashSet;

use crate::helpers::parse_input;

pub fn part_1(input: &str) -> f64 {
    let machines = parse_input(input, 0.0);

    let mut computed_found = HashSet::new();
    let mut computed_total_cost = 0.0;

    for (i, machine) in machines.iter().enumerate() {
        if let Some((a, b)) = machine.compute() {
            let a_cost = a * 3.0;
            computed_total_cost += a_cost + b;
            computed_found.insert(i);
        }
    }

    computed_total_cost
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 480.0);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 31761.0);
    }
}
