use crate::warehouse_2x::Warehouse2X;

pub fn part_2(input: &str) -> i32 {
    let mut warehouse = Warehouse2X::new(input);
    warehouse.run();
    warehouse.get_gps_cooridnate_sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_2_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 9021);
    }

    #[test]
    fn part_2_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
