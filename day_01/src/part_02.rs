pub fn part_2(input: &str) -> i32 {
    let lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let split: Vec<_> = line.split("   ").collect();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    let mut sum = 0;

    for l in left {
        let occurences = right.iter().filter(|&r| r == &l).count();
        sum += occurences as i32 * l;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_example_test() {
        let input = fs::read_to_string("./test_input.txt").expect("Failed to read input file");
        let output = part_2(&input);
        assert_eq!(output, 31);
    }

    #[test]
    fn part_2_test() {
        let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
        let output = part_2(&input);
        dbg!(output);
    }
}
