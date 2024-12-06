use core::time;
use std::{collections::HashSet, thread};

use crate::map::{Map, Space};

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
    visited: HashSet<String>,
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
            map,
            print_map: false,
        }
    }

    pub fn with_print_map(mut self, value: bool) -> Self {
        self.print_map = value;
        self
    }

    pub fn patrol(&mut self) -> i32 {
        self.reset();
        self.print_map();

        loop {
            self.visited.insert(pos_to_string((self.x, self.y)));

            let next_space = self.get_next_space();
            let next_space_type = self.map.get_position(next_space.0, next_space.1);

            if next_space_type.is_none() {
                break;
            }

            if let Some(Space::Obstacle) = next_space_type {
                self.rotate();
            }

            let next_space = self.get_next_space();
            self.x = next_space.0;
            self.y = next_space.1;

            self.print_map();
        }

        self.print_map();

        self.visited.len() as i32
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
                } else {
                    let pos = pos_to_string((x, y));
                    if self.visited.contains(&pos) {
                        print!("o");
                    } else {
                        match self.map.get_position(x, y) {
                            Some(Space::Open) => print!("."),
                            Some(Space::Obstacle) => print!("#"),
                            None => print!(" "),
                        }
                    }
                }
            }
            println!();
        }

        thread::sleep(time::Duration::from_millis(400));
        self.clear_screen();
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

fn pos_to_string(pos: (i32, i32)) -> String {
    format!("{},{}", pos.0, pos.1)
}
