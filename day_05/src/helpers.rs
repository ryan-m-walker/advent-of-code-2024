use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
};

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

        let current_set = ordering_map.entry(second).or_insert_with(HashSet::new);
        current_set.insert(first);
    }

    let mut updates = vec![];

    for line in update_section.lines() {
        let pages = line.split(",").map(|x| x.parse().unwrap()).collect();
        updates.push(pages);
    }

    (ordering_map, updates)
}

pub fn is_correct_order(order_map: &OrderingMap, update: &Vec<i32>) -> bool {
    let all: HashSet<i32, RandomState> = HashSet::from_iter(update.iter().copied());
    let mut seen = HashSet::new();

    for page in update {
        seen.insert(*page);

        if !satisfies_requirements(*page, &all, &seen, order_map) {
            return false;
        }
    }

    true
}

pub fn satisfies_requirements(
    page: i32,
    all: &HashSet<i32>,
    seen: &HashSet<i32>,
    order_map: &OrderingMap,
) -> bool {
    let Some(requirments) = order_map.get(&page) else {
        return true;
    };

    for requirment in requirments {
        if !all.contains(requirment) {
            continue;
        }

        if !seen.contains(requirment) {
            return false;
        }
    }

    true
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
