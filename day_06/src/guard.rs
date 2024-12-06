use core::time;
use std::{collections::HashSet, thread};

use crate::map::{Map, Space};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Guard<'a> {
    x: i32,
    y: i32,
    direction: Direction,
    visited: HashSet<(i32, i32)>,
    visit_with_direction: HashSet<String>,
    map: &'a Map,
    print_map: bool,
}

impl<'a> Guard<'a> {
    pub fn new(map: &'a Map) -> Self {
        Guard {
            x: map.starting_x,
            y: map.starting_y,
            direction: Direction::Up,
            visited: HashSet::new(),
            visit_with_direction: HashSet::new(),
            map,
            print_map: false,
        }
    }

    pub fn with_print_map(mut self, value: bool) -> Self {
        self.print_map = value;
        self
    }

    pub fn patrol(&mut self) -> Option<HashSet<(i32, i32)>> {
        self.reset();
        self.print_map();

        loop {
            self.visited.insert((self.x, self.y));

            let visit_with_direction =
                visit_with_direction_to_string(self.x, self.y, &self.direction);

            if self.visit_with_direction.contains(&visit_with_direction) {
                return None;
            }

            self.visit_with_direction
                .insert(visit_with_direction_to_string(
                    self.x,
                    self.y,
                    &self.direction,
                ));

            let next_space = self.get_next_space();
            let next_space_type = self.map.get_position(next_space.0, next_space.1);

            if next_space_type.is_none() {
                break;
            }

            if let Some(Space::Obstacle) = next_space_type {
                self.rotate();
                continue;
            }

            let next_space = self.get_next_space();
            self.x = next_space.0;
            self.y = next_space.1;

            self.print_map();
        }

        self.print_map();

        Some(self.visited.clone())
    }

    fn reset(&mut self) {
        self.x = self.map.starting_x;
        self.y = self.map.starting_y;
        self.direction = Direction::Up;
        self.visited.clear();
    }

    fn get_next_space(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn print_map(&self) {
        if !self.print_map {
            return;
        }

        print!("\x1B[2J\x1B[1;1H");

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if x == self.x && y == self.y {
                    print!("{}", self.get_guard_char());
                    continue;
                }

                if self.visited.contains(&(x, y)) {
                    print!("o");
                    continue;
                }

                if let Some(obstacle) = self.map.obstacle {
                    if x == obstacle.0 && y == obstacle.1 {
                        print!("O");
                        continue;
                    }
                }

                match self.map.get_position(x, y) {
                    Some(Space::Open) => print!("."),
                    Some(Space::Obstacle) => print!("#"),
                    None => print!(" "),
                }
            }
            println!();
        }

        println!();
        println!();
        thread::sleep(time::Duration::from_millis(200));
    }

    fn get_guard_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

fn visit_with_direction_to_string(x: i32, y: i32, direction: &Direction) -> String {
    format!("({}:{}:{:?})", x, y, direction)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Map;
    use super::*;

    #[test]
    fn loop_break_test() {
        let input = fs::read_to_string("./test_input_1.txt").unwrap();
        let map = Map::new(&input, None);
        let mut guard = Guard::new(&map);
        let output = guard.patrol();
        assert_eq!(output, None);
    }

    #[test]
    fn multi_turn_test() {
        let input = fs::read_to_string("./test_input_2.txt").unwrap();
        let map = Map::new(&input, None);
        let mut guard = Guard::new(&map);
        let output = guard.patrol().unwrap().len();
        assert_eq!(output, 4);
    }

    #[test]
    fn addition_test() {
        let input = fs::read_to_string("./test_input_3.txt").unwrap();
        let map = Map::new(&input, Some((5, 1)));
        let mut guard = Guard::new(&map);
        let output = guard.patrol();
        assert_eq!(output, None);
    }
}
