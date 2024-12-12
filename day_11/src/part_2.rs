use crate::stones::Stones;

pub fn part_2(input: &str) -> i64 {
    let mut stones = Stones::new(input);
    stones.blink_n_times(75)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_4() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let output = part_2(&input);
        dbg!(output);
    }
}
