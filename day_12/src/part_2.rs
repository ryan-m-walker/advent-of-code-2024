use crate::farm::Farm;

pub fn part_2(input: &str) -> i32 {
    let farm = Farm::new(input);
    let regions = farm.get_regions();

    let mut output = 0;

    for region in regions {
        output += region.get_fence_edges().len() as i32 * region.area();
    }

    output
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_2_simple_input() {
        let input = ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n");
        let output = part_2(&input);
        assert_eq!(output, 80);
    }

    #[test]
    fn part_2_e_shaped_input() {
        let input = fs::read_to_string("./e_shaped_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 236);
    }

    #[test]
    fn part_2_a_b_input() {
        let input = ["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"].join("\n");
        let output = part_2(&input);
        assert_eq!(output, 368);
    }

    #[test]
    fn part_2_example_input() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 1206);
    }

    #[test]
    fn part_2_input() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
