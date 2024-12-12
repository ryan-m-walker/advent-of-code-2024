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

    pub fn neighbours(&self) -> Vec<(i32, i32)> {
        let up = (self.x, self.y - 1);
        let right = (self.x + 1, self.y);
        let down = (self.x, self.y + 1);
        let left = (self.x - 1, self.y);

        vec![up, right, down, left]
    }
}
