use crate::warehouse::Warehouse;

pub fn part_1(input: &str) -> i32 {
    let mut warehouse = Warehouse::new(input);
    warehouse.run();
    warehouse.print();
    warehouse.get_gps_cooridnate_sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test_input() {
        let input = fs::read_to_string("./test_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 2028);
    }

    #[test]
    fn part_1_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 10092);
    }

    #[test]
    fn part_1_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_1(&input);
        dbg!(output);
    }
}
