use crate::map::Map;

pub fn day_1(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.run()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn day_1_example_test() {
        let input = fs::read_to_string("./example_input.txt").unwrap();
        let output = day_1(&input);
        assert_eq!(output, 14)
    }

    #[test]
    fn map_parse_test_3() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = day_1(&input);
        dbg!(output);
    }
}
