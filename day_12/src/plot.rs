use crate::region::Dir;

#[derive(Debug, Clone, Copy)]
pub struct Plot {
    pub x: i32,
    pub y: i32,
    pub plant: char,
}

impl Plot {
    pub fn new(x: i32, y: i32, plant: char) -> Self {
        Self { x, y, plant }
    }

    pub fn neighbours(&self) -> Vec<(i32, i32, Dir)> {
        let up = (self.x, self.y - 1, Dir::Up);
        let right = (self.x + 1, self.y, Dir::Right);
        let down = (self.x, self.y + 1, Dir::Down);
        let left = (self.x - 1, self.y, Dir::Left);

        vec![up, right, down, left]
    }
}
