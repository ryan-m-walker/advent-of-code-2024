use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Region {
    pub plant: char,
    pub area: i64,
    pub parimeter: i64,

    plots: HashSet<(i32, i32)>,
}

impl Region {
    pub fn new(plant: char) -> Self {
        Self {
            plant,
            plots: HashSet::new(),
            area: 0,
            parimeter: 0,
        }
    }

    pub fn add(&mut self, x: i32, y: i32) {
        if self.plots.contains(&(x, y)) {
            return;
        }

        self.plots.insert((x, y));
        self.area += 1;
    }

    pub fn has(&self, x: i32, y: i32) -> bool {
        self.plots.contains(&(x, y))
    }

    pub fn increment_parimeter(&mut self) {
        self.parimeter += 1;
    }
}
