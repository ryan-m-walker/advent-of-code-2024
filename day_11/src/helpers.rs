pub fn rule_1(stone: &str) -> bool {
    stone == "0"
}

pub fn rule_2(stone: &str) -> bool {
    stone.len() % 2 == 0
}

pub fn blink(stones: &Vec<String>) -> Vec<String> {
    let mut next: Vec<String> = Vec::new();

    for stone in stones {
        if rule_1(stone) {
            next.push(String::from("1"));
            continue;
        }

        if rule_2(stone) {
            let halfway = stone.len() / 2;
            let left = &stone[..halfway];
            let right = &stone[halfway..];

            let left_parsed: i64 = left.parse().unwrap();
            let right_parsed: i64 = right.parse().unwrap();

            next.push(left_parsed.to_string());
            next.push(right_parsed.to_string());
            continue;
        }

        let parsed: i64 = stone.parse().unwrap();
        let multiplied = parsed * 2024;
        let new_stone = multiplied.to_string();

        next.push(new_stone);
    }

    next
}

pub fn blink_n_times(stones: Vec<String>, n: i32) -> Vec<String> {
    let mut current = stones;

    for _ in 0..n {
        current = blink(&current);
    }

    current
}

pub fn parse_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_1() {
        assert!(rule_1("0"));

        assert!(!rule_1("1"));
        assert!(!rule_1("1000"));
    }

    #[test]
    fn test_rule_2() {
        assert!(rule_2("10"));
        assert!(rule_2("904831"));

        assert!(!rule_2("0"));
        assert!(!rule_2("123"));
    }

    #[test]
    fn test_blink() {
        let input = parse_input("0 1 10 99 999");
        let expected = parse_input("1 2024 1 0 9 9 2021976");
        let output = blink(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_blink_n_times() {
        let input = parse_input("125 17");
        let expected =
            parse_input("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
        let output = blink_n_times(input, 6);
        assert_eq!(output, expected);
    }
}
