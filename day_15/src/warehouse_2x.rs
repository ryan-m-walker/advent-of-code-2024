use std::{thread, time};

use crate::helpers::{parse_directions, Direction};
use colored::Colorize;

#[derive(Debug, PartialEq, Copy, Clone)]
enum EntityType {
    Box,
    Wall,
}

#[derive(Debug, PartialEq)]
struct Entity {
    index: usize,
    entity_type: EntityType,
    x: i32,
    y: i32,
    width: i32,
}

pub struct Warehouse2X {
    entities: Vec<Entity>,
    directions: Vec<Direction>,
    robot: (i32, i32),
    width: i32,
    height: i32,
    should_print: bool,
}

impl Warehouse2X {
    pub fn new(input: &str) -> Self {
        let split = input.split("\n\n").collect::<Vec<&str>>();
        let map = split[0].trim();
        let directions = parse_directions(split[1].trim());

        let mut entities = Vec::new();
        let mut robot = (0, 0);

        let mut x = 0;
        let mut y = 0;
        let mut width = 0;

        for c in map.chars() {
            if c == '\n' {
                if width == 0 {
                    width = x;
                }

                x = 0;
                y += 1;
                continue;
            }

            if c == '@' {
                robot = (x, y);
            } else if c == '#' {
                entities.push(Entity {
                    index: entities.len(),
                    entity_type: EntityType::Wall,
                    width: 2,
                    x,
                    y,
                })
            } else if c == 'O' {
                entities.push(Entity {
                    index: entities.len(),
                    entity_type: EntityType::Box,
                    width: 2,
                    x,
                    y,
                })
            }

            x += 2;
        }

        Self {
            entities,
            robot,
            directions,
            width,
            height: y + 1,
            should_print: false,
        }
    }

    pub fn with_print(mut self, value: bool) -> Self {
        self.should_print = value;
        self
    }

    pub fn print(&self) {
        // clear
        // print!("\x1B[2J\x1B[1;1H");

        for y in 0..self.height {
            let mut x = 0;

            while x < self.width {
                if x == self.robot.0 && y == self.robot.1 {
                    print!("{}", "@".red().bold());
                    x += 1;
                    continue;
                }

                let Some(entity) = self.get_entity((x, y)) else {
                    print!("{}", ".".dimmed());
                    x += 1;
                    continue;
                };

                match entity.entity_type {
                    EntityType::Box => {
                        print!("{}", "[]".yellow());
                    }
                    EntityType::Wall => {
                        print!("##");
                    }
                }

                x += 2;
            }

            println!();
        }

        println!();
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
                self.print();
                println!(
                    "{}",
                    format!("Step {} / {}:", i + 1, self.directions.len()).italic()
                );
                println!(
                    "{}",
                    format!("Direction {}", self.print_direction(&direction)).bold()
                );
                thread::sleep(time::Duration::from_millis(1000));
            }
        }
    }

    // 193

    fn step(&mut self, direction: &Direction) {
        let next_space = self.get_next_space(self.robot, direction);

        let Some(entity) = self.get_entity(next_space) else {
            self.robot = next_space;
            return;
        };

        if let EntityType::Box = entity.entity_type {
            if let Some(box_updates) = self.push(next_space, direction) {
                self.robot = next_space;

                for update in box_updates {
                    let entity = self.entities.get_mut(update.index).unwrap();
                    *entity = update;
                }
            }
        }
    }

    fn push(&self, space: (i32, i32), direction: &Direction) -> Option<Vec<Entity>> {
        let Some(entity) = self.get_entity(space) else {
            return Some(vec![]);
        };

        if entity.entity_type == EntityType::Wall {
            return None;
        }

        let mut box_updates: Vec<Entity> = vec![];

        match direction {
            Direction::Right => {
                let coord = (space.0 + 1, space.1);
                if let Some(other_updates) = self.push(coord, direction) {
                    box_updates.push(Entity {
                        index: entity.index,
                        entity_type: EntityType::Box,
                        x: entity.x + 1,
                        y: entity.y,
                        width: entity.width,
                    });
                    box_updates.extend(other_updates);
                    return Some(box_updates);
                }
            }
            Direction::Left => {
                let coord = (space.0 - 1, space.1);
                if let Some(other_updates) = self.push(coord, direction) {
                    box_updates.push(Entity {
                        index: entity.index,
                        entity_type: EntityType::Box,
                        x: entity.x - 1,
                        y: entity.y,
                        width: entity.width,
                    });
                    box_updates.extend(other_updates);
                    return Some(box_updates);
                }
            }
            Direction::Up => {
                let coord_left = (entity.x, entity.y - 1);
                let coord_right = (entity.x + 1, entity.y - 1);

                let push_left = self.push(coord_left, direction);
                let push_right = self.push(coord_right, direction);

                if push_left.is_some() && push_right.is_some() {
                    box_updates.push(Entity {
                        index: entity.index,
                        entity_type: EntityType::Box,
                        x: entity.x,
                        y: entity.y - 1,
                        width: entity.width,
                    });

                    if let Some(push_left) = push_left {
                        box_updates.extend(push_left);
                    }

                    if let Some(push_right) = push_right {
                        box_updates.extend(push_right);
                    }

                    return Some(box_updates);
                }
            }
            Direction::Down => {
                let coord_left = (entity.x, entity.y + 1);
                let coord_right = (entity.x + 1, entity.y + 1);

                let push_left = self.push(coord_left, direction);
                let push_right = self.push(coord_right, direction);

                if push_left.is_some() && push_right.is_some() {
                    box_updates.push(Entity {
                        index: entity.index,
                        entity_type: EntityType::Box,
                        x: entity.x,
                        y: entity.y + 1,
                        width: entity.width,
                    });

                    if let Some(push_left) = push_left {
                        box_updates.extend(push_left);
                    }

                    if let Some(push_right) = push_right {
                        box_updates.extend(push_right);
                    }

                    return Some(box_updates);
                }
            }
        }

        None
    }

    fn get_next_space(&self, space: (i32, i32), direction: &Direction) -> (i32, i32) {
        let (x, y) = space;

        match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn get_entity(&self, space: (i32, i32)) -> Option<&Entity> {
        let (x, y) = space;

        for entity in &self.entities {
            if y != entity.y {
                continue;
            }

            if x >= entity.x && x < entity.x + entity.width {
                return Some(entity);
            }
        }

        None
    }

    fn print_direction(&self, direction: &Direction) -> String {
        match direction {
            Direction::Up => "^".green().to_string(),
            Direction::Down => "v".green().to_string(),
            Direction::Left => "<".green().to_string(),
            Direction::Right => ">".green().to_string(),
        }
    }

    pub fn get_gps_cooridnate_sum(&self) -> i32 {
        let mut sum = 0;

        for entity in &self.entities {
            if entity.entity_type == EntityType::Box {
                sum += entity.y * 100 + entity.x;
            }
        }

        sum
    }
}
