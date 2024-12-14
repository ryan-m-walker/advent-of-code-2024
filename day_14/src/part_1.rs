pub fn part_1(input: &str) -> i32 {
    unimplemented!();
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
}
