pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_input_works() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = parse_input(&input);

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(output, expected);
    }
}
