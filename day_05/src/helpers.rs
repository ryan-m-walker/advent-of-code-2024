use std::collections::{HashMap, HashSet};

type OrderingMap = HashMap<i32, HashSet<i32>>;
type Updates = Vec<Vec<i32>>;

pub fn parse_input(input: &str) -> (OrderingMap, Updates) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let ordering_rules_section = split[0];
    let update_section = split[1];

    let mut ordering_map = HashMap::new();

    for line in ordering_rules_section.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let first: i32 = parts[0].parse().unwrap();
        let second: i32 = parts[1].parse().unwrap();

        if !ordering_map.contains_key(&second) {
            ordering_map.insert(second, HashSet::new());
        }

        let current_set = ordering_map.get_mut(&second).unwrap();
        current_set.insert(first);
    }

    let mut updates = vec![];

    for line in update_section.lines() {
        let pages = line.split(",").map(|x| x.parse().unwrap()).collect();
        updates.push(pages);
    }

    (ordering_map, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let input = ["1|2", "", "1,2"].join("\n");

        let mut expected_set = HashSet::new();
        expected_set.insert(1);

        let mut expected_map = HashMap::new();
        expected_map.insert(2, expected_set);

        let expected_updates = vec![vec![1, 2]];

        let (ordering_map, updates) = parse_input(&input);

        assert_eq!(ordering_map, expected_map);
        assert_eq!(updates, expected_updates);
    }
}
