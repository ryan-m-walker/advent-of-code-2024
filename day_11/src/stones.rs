use std::collections::HashMap;

use crate::helpers::{parse_input, rule_1, rule_2};

pub struct Stones {
    stones: Vec<String>,
    cache_hits: i64,
    cache: HashMap<String, i64>,
}

impl Stones {
    pub fn new(input: &str) -> Self {
        Self {
            stones: parse_input(input),
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }

    pub fn blink_n_times(&mut self, n: i32) -> i64 {
        let mut output = 0;

        for s in self.stones.clone() {
            output += self.blink(&s, n);
        }

        output
    }

    fn get_cached_value(&mut self, cache_key: &str) -> Option<i64> {
        let value = self.cache.get(cache_key).copied();

        if value.is_some() {
            self.cache_hits += 1;
        }

        value
    }

    fn get_cache_key(&self, stone: &str, n: i32) -> String {
        format!("{}-{}", stone, n)
    }

    fn blink(&mut self, stone: &str, n: i32) -> i64 {
        if n <= 0 {
            let cache_key = self.get_cache_key(stone, n);
            self.cache.insert(cache_key, 1);
            return 1;
        }

        if rule_1(stone) {
            return self.blink("1", n - 1);
        }

        if rule_2(stone) {
            let halfway = stone.len() / 2;
            let left = &stone[..halfway];
            let right = &stone[halfway..];

            let left_parsed: i64 = left.parse().unwrap();
            let right_parsed: i64 = right.parse().unwrap();

            let cache_key_1 = self.get_cache_key(&left_parsed.to_string(), n - 1);
            let output_1 = self.blink(&left_parsed.to_string(), n - 1);
            self.cache.insert(cache_key_1, output_1);

            let cache_key_2 = self.get_cache_key(&right_parsed.to_string(), n - 1);
            let output_2 = self.blink(&right_parsed.to_string(), n - 1);
            self.cache.insert(cache_key_2, output_2);

            return output_1 + output_2;
        }

        let parsed: i64 = stone.parse().unwrap();
        let multiplied = parsed * 2024;
        let new_stone = multiplied.to_string();

        let cache_key = self.get_cache_key(stone, n);

        if let Some(v) = self.get_cached_value(&cache_key) {
            return v;
        }

        self.blink(&new_stone, n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "125 17";
        let mut stones = Stones::new(input);
        let output = stones.blink_n_times(6);
        assert_eq!(output, 22);
    }
}
