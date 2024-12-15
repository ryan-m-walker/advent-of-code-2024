use colored::Colorize;
use std::collections::HashMap;
use std::thread;
use std::time;

use crate::helpers::Direction;

#[derive(Debug, PartialEq)]
pub enum Entity {
    Box,
    Wall,
}

#[derive(Debug)]
pub struct Warehouse {
    pub entities: HashMap<(i32, i32), Entity>,
    pub directions: Vec<Direction>,
    pub width: i32,
    pub height: i32,
    pub robot: (i32, i32),

    should_print: bool,
}

impl Warehouse {
    pub fn new(input: &str) -> Self {
        let split = input.split("\n\n").collect::<Vec<&str>>();
        let map_chars = split[0].trim().chars();
        let dir_chars = split[1].trim().chars();

        let mut directions = Vec::new();

        for char in dir_chars {
            match char {
                '^' => directions.push(Direction::Up),
                'v' => directions.push(Direction::Down),
                '<' => directions.push(Direction::Left),
                '>' => directions.push(Direction::Right),
                _ => {}
            }
        }

        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut robot = (0, 0);

        let mut entities = HashMap::new();

        for char in map_chars {
            if char == '\n' {
                if width == 0 {
                    width = x;
                }

                y += 1;
                x = 0;

                continue;
            }

            if char == '#' {
                entities.insert((x, y), Entity::Wall);
            } else if char == '@' {
                robot = (x, y);
            } else if char == 'O' {
                entities.insert((x, y), Entity::Box);
            }

            x += 1;
        }

        Self {
            entities,
            directions,
            width,
            height: y + 1,
            robot,
            should_print: false,
        }
    }

    pub fn with_print(mut self, value: bool) -> Self {
        self.should_print = value;
        self
    }

    pub fn run(&mut self) {
        if self.should_print {
            println!();
            println!("{}", "Initial state:".italic());
            self.print();
        }

        for i in 0..self.directions.len() {
            let direction = self.directions[i];
            self.step(&direction);

            if self.should_print {
                println!("{}", format!("Step {}:", i + 1).italic());
                self.print();
                thread::sleep(time::Duration::from_millis(300));
            }
        }
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..self.height {
            for x in 0..self.width {
                if self.robot == (x, y) {
                    print!("{}", "@".red().bold());
                } else {
                    match self.entities.get(&(x, y)) {
                        Some(Entity::Wall) => print!("#"),
                        Some(Entity::Box) => print!("{}", "O".yellow()),
                        None => print!("{}", ".".dimmed()),
                    }
                }
            }

            println!();
        }

        println!();
    }

    pub fn get_gps_cooridnate_sum(&self) -> i32 {
        let mut sum = 0;

        for (position, entity) in &self.entities {
            if *entity == Entity::Wall {
                continue;
            }

            let (x, y) = position;
            sum += y * 100 + x;
        }

        sum
    }

    fn step(&mut self, direction: &Direction) {
        let next_space = self.get_next_space(self.robot, direction);

        match self.entities.get(&next_space) {
            Some(Entity::Wall) => {}
            Some(Entity::Box) => {
                if self.push(next_space, direction) {
                    self.robot = next_space;
                }
            }
            None => {
                self.robot = next_space;
            }
        }
    }

    fn push(&mut self, space: (i32, i32), direction: &Direction) -> bool {
        let next_space = self.get_next_space(space, direction);

        match self.entities.get(&next_space) {
            Some(Entity::Wall) => false,
            Some(Entity::Box) => {
                if self.push(next_space, direction) {
                    self.entities.remove(&space);
                    self.entities.insert(next_space, Entity::Box);
                    true
                } else {
                    false
                }
            }
            None => {
                self.entities.remove(&space);
                self.entities.insert(next_space, Entity::Box);
                true
            }
            _ => false,
        }
    }

    /// since the whole warehouse is walled in we can probably assume all spaces are in bounds
    fn get_next_space(&self, space: (i32, i32), direction: &Direction) -> (i32, i32) {
        let (x, y) = space;

        match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}
