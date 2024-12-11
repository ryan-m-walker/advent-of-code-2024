use std::collections::HashMap;

use crate::helpers::{parse_input, rule_1, rule_2};

pub struct Stones {
    stones: Vec<String>,
    visits: HashMap<String, String>,
}

impl Stones {
    pub fn new(input: &str) -> Self {
        Self {
            stones: parse_input(input),
            visits: HashMap::new(),
        }
    }

    pub fn blink_n_times(&mut self, n: i32) -> i64 {
        let mut output = String::new();

        for stone in self.stones.clone() {
            dbg!(&stone);

            if !output.is_empty() {
                output.push(' ');
            }

            output.push_str(&self.blink_stone_n_times(&stone, n));
        }

        output.split(' ').count() as i64
    }

    fn blink_stone_n_times(&mut self, stone: &str, n: i32) -> String {
        let mut next = stone.to_string();

        for i in 0..n {
            if self.visits.contains_key(&next) {
                return self.visits.get(stone).unwrap().to_string();
            }

            dbg!(i);

            next = self.blink_stone(&next);
            self.visits.insert(stone.to_string(), next.clone());
        }

        next
    }

    fn blink(&self, stone: &str, n: i32) {}

    fn blink_stone(&mut self, stone: &str) -> String {
        if self.visits.contains_key(stone) {
            return self.visits.get(stone).unwrap().to_string();
        }

        // dbg!(self.visits.len());

        let mut output = Vec::new();
        let stones = parse_input(stone);

        for stone in stones {
            if self.visits.contains_key(&stone) {
                output.push(self.visits.get(&stone).unwrap().to_string());
                continue;
            }

            if rule_1(&stone) {
                output.push(String::from("1"));
                self.visits.insert(stone.to_string(), String::from("1"));
                continue;
            }

            if rule_2(&stone) {
                let halfway = stone.len() / 2;
                let left = &stone[..halfway];
                let right = &stone[halfway..];

                let left_parsed: i64 = left.parse().unwrap();
                let right_parsed: i64 = right.parse().unwrap();

                output.push(left_parsed.to_string());
                output.push(right_parsed.to_string());

                self.visits.insert(
                    stone.to_string(),
                    format!("{} {}", left_parsed, right_parsed),
                );
                continue;
            }

            let parsed: i64 = stone.parse().unwrap();
            let multiplied = parsed * 2024;
            output.push(multiplied.to_string());
            self.visits
                .insert(stone.to_string(), multiplied.to_string());
        }

        let output = output.join(" ");

        self.visits.insert(stone.to_string(), output.clone());

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_name() {
    //     let input = "125 17";
    //     let mut stones = Stones::new(input);
    //     let output = stones.blink_n_times(6);
    //     assert_eq!(output, 22,);
    // }
    //
    // #[test]
    // fn test_name_2() {
    //     let input = "125 17";
    //     let mut stones = Stones::new(input);
    //     let output = stones.blink_n_times(25);
    //     assert_eq!(output, 55312);
    // }

    #[test]
    fn test_name_2() {
        let input = "125";
        let mut stones = Stones::new(input);
        let output = stones.blink_n_times(75);
        dbg!(output);
    }
}
