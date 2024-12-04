use std::iter::zip;

pub fn part_01(input: &str) -> i32 {
    let lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let split: Vec<_> = line.split("   ").collect();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| (r - l).abs()).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_test() {
        let input = fs::read_to_string("./test_input.txt").expect("Failed to read input file");
        let output = part_01(&input);
        assert_eq!(output, 11);
    }

    #[test]
    fn part_1_test() {
        let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
        let output = part_01(&input);
        dbg!(output);
    }
}
