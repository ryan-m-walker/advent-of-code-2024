pub fn rule_1(stone: &str) -> bool {
    stone == "0"
}

pub fn rule_2(stone: &str) -> bool {
    stone.len() % 2 == 0
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
}
