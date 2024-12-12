use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub plant: char,
    pub parimeter: i32,
    pub fences: HashMap<String, Vec<i32>>,
    pub plots: HashSet<(i32, i32)>,
}

impl Region {
    pub fn new(plant: char) -> Self {
        Self {
            plant,
            plots: HashSet::new(),
            parimeter: 0,
            fences: HashMap::new(),
        }
    }

    pub fn add(&mut self, x: i32, y: i32) {
        if self.plots.contains(&(x, y)) {
            return;
        }

        self.plots.insert((x, y));
    }

    pub fn has(&self, x: i32, y: i32) -> bool {
        self.plots.contains(&(x, y))
    }

    pub fn area(&self) -> i32 {
        self.plots.len() as i32
    }

    pub fn add_fence(&mut self, dir: Dir, x: i32, y: i32) {
        self.parimeter += 1;

        match dir {
            Dir::Up => {
                let key = self.get_fence_key(dir, y + 1);
                self.fences.entry(key).or_default().push(x);
            }
            Dir::Down => {
                let key = self.get_fence_key(dir, y - 1);
                self.fences.entry(key).or_default().push(x);
            }
            Dir::Right => {
                let key = self.get_fence_key(dir, x - 1);
                self.fences.entry(key).or_default().push(y);
            }
            Dir::Left => {
                let key = self.get_fence_key(dir, x + 1);
                self.fences.entry(key).or_default().push(y);
            }
        }
    }

    pub fn get_fence_edges(&self) -> Vec<Vec<(String, i32)>> {
        let mut edges = vec![];

        for (key, values) in &self.fences {
            let mut values_clone = values.clone();
            values_clone.sort();

            let mut current = vec![];

            for (i, fence) in values_clone.iter().enumerate() {
                current.push((key.clone(), *fence));

                let Some(next) = values_clone.get(i + 1) else {
                    edges.push(current);
                    current = vec![];
                    continue;
                };

                let diff = fence.abs_diff(*next);

                if diff > 1 {
                    edges.push(current);
                    current = vec![];
                    continue;
                }
            }
        }

        edges
    }

    fn get_fence_key(&self, dir: Dir, position: i32) -> String {
        format!("{:?}({})", dir, position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut region = Region::new('A');
        region.add_fence(Dir::Up, 0, 0);
        region.add_fence(Dir::Up, 0, 1);
        region.add_fence(Dir::Up, 0, 2);
        region.get_fence_edges();
    }
}
