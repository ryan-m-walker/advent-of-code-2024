pub fn parse_input(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = parse_input(&input);
    }
}
