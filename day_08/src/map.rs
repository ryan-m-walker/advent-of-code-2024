use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub data: Vec<char>,
    pub frequencies: HashMap<String, Vec<(i32, i32)>>,
    pub nodes: HashSet<(i32, i32)>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut frequencies = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;

        let mut data = vec![];

        for c in input.chars() {
            if c == '\n' {
                if width == 0 {
                    width = x;
                }

                y += 1;
                x = 0;

                continue;
            }

            if c != '.' {
                let freq = frequencies.entry(c.to_string()).or_insert(Vec::new());
                freq.push((x, y));
            }

            x += 1;
            data.push(c);
        }

        Map {
            width,
            height: y + 1,
            data,
            frequencies,
            nodes: HashSet::new(),
        }
    }

    pub fn get_single_antinodes(&mut self) -> i32 {
        for freq in self.frequencies.values() {
            for antenna in freq {
                for other in freq {
                    if other.0 == antenna.0 && other.1 == antenna.1 {
                        continue;
                    }

                    let dx = other.0 - antenna.0;
                    let dy = other.1 - antenna.1;

                    let node_x = antenna.0 + dx * 2;
                    let node_y = antenna.1 + dy * 2;

                    let pos = self.get(node_x, node_y);
                    if pos.is_none() {
                        continue;
                    }

                    self.nodes.insert((node_x, node_y));
                }
            }
        }

        self.nodes.len() as i32
    }

    pub fn get_all_antinodes(&mut self) -> i32 {
        for freq in self.frequencies.values() {
            for antenna in freq {
                for other in freq {
                    if other.0 == antenna.0 && other.1 == antenna.1 {
                        continue;
                    }

                    let mut offset = 1;

                    let dx = other.0 - antenna.0;
                    let dy = other.1 - antenna.1;

                    loop {
                        let node_x = antenna.0 + dx * offset;
                        let node_y = antenna.1 + dy * offset;

                        let pos = self.get(node_x, node_y);
                        if pos.is_none() {
                            break;
                        }

                        self.nodes.insert((node_x, node_y));
                        offset += 1;
                    }
                }
            }
        }

        self.nodes.len() as i32
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.nodes.contains(&(x, y)) {
                    print!("#");
                    continue;
                }

                if let Some(c) = self.get(x, y) {
                    print!("{}", c);
                    continue;
                }
            }

            println!();
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        let index = (y * self.width + x) as usize;
        self.data.get(index)
    }

    fn get_antinode(&self, x: i32, y: i32, offset: i32) -> Option<&char> {
        let dx = x + offset;
        let dy = y + offset;

        if dx < 0 || dy < 0 || dx >= self.width || dy >= self.height {
            return None;
        }

        self.get(dx, dy)
    }
}
