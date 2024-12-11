use std::collections::HashMap;

pub fn rule_1(stone: &str) -> bool {
    stone == "0"
}

pub fn rule_2(stone: &str) -> bool {
    stone.len() % 2 == 0
}

pub fn blink_all(stones: Vec<String>) -> Vec<String> {
    let mut next: Vec<String> = Vec::new();
    let mut zero_count = 0;

    for stone in stones {
        let (stones, zero_found) = blink(&stone);

        if zero_found {
            zero_count += 1;
            // println!("zeros found = {}", zero_count);
        }

        for s in stones {
            next.push(s);
        }
    }

    // println!("zeros found = {}", zero_count);

    next
}

pub fn blink(stone: &str) -> (Vec<String>, bool) {
    if rule_1(stone) {
        return (vec![String::from("1")], true);
    }

    if rule_2(stone) {
        let halfway = stone.len() / 2;
        let left = &stone[..halfway];
        let right = &stone[halfway..];

        let left_parsed: i64 = left.parse().unwrap();
        let right_parsed: i64 = right.parse().unwrap();

        return (
            vec![left_parsed.to_string(), right_parsed.to_string()],
            false,
        );
    }

    let parsed: i64 = stone.parse().unwrap();
    let multiplied = parsed * 2024;
    let new_stone = multiplied.to_string();

    (vec![new_stone], false)
}

pub fn blink_str(stone: &str) -> String {
    if rule_1(stone) {
        return String::from("1");
    }

    if rule_2(stone) {
        let halfway = stone.len() / 2;
        let left = &stone[..halfway];
        let right = &stone[halfway..];

        let left_parsed: i64 = left.parse().unwrap();
        let right_parsed: i64 = right.parse().unwrap();

        return format!("{} {}", left_parsed, right_parsed);
    }

    dbg!(stone);
    let parsed: i64 = stone.parse().unwrap();
    let multiplied = parsed * 2024;
    multiplied.to_string()
}

pub fn blink_n_times(stones: Vec<String>, n: i32) -> Vec<String> {
    let mut current = stones;

    for _ in 0..n {
        current = blink_all(current);

        println!("{}", current.join(" "));
    }

    current
}

pub fn blink_n_times_part_2(stones: Vec<String>, n: i32) -> String {
    stones.iter().fold(String::from(""), |mut acc, stone| {
        let next = blink_n_times_recursive(stone, n);
        acc.push(' ');
        acc.push_str(next.as_str());
        acc
    })
}

pub fn blink_n_times_recursive(stone: &str, n: i32) -> String {
    println!("blink: #{}, stone: {}", n, stone);

    if n == 0 {
        return stone.to_string();
    }
    let next = blink_str(stone);
    blink_n_times_recursive(&next, n - 1)
}

pub fn parse_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_rule_1() {
    //     assert!(rule_1("0"));
    //
    //     assert!(!rule_1("1"));
    //     assert!(!rule_1("1000"));
    // }
    //
    // #[test]
    // fn test_rule_2() {
    //     assert!(rule_2("10"));
    //     assert!(rule_2("904831"));
    //
    //     assert!(!rule_2("0"));
    //     assert!(!rule_2("123"));
    // }

    #[test]
    fn test_blink() {
        let input = parse_input("0");
        let _output = blink_n_times(input, 10);
    }

    // #[test]
    // fn test_blink_2() {
    //     let input = parse_input("0 1 10 99 999");
    //     let expected = "1 2024 1 0 9 9 2021976";
    //     let output = blink_n_times_part_2(input, 75);
    //     assert_eq!(output, expected);
    // }

    // #[test]
    // fn test_blink_n_times() {
    //     let input = parse_input("125 17");
    //     let expected =
    //         parse_input("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
    //     let output = blink_n_times(input, 75);
    //     assert_eq!(output, expected);
    // }
}
