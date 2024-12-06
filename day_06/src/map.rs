#[derive(Debug)]
pub struct Map {
    data: Vec<Space>,
    pub starting_x: i32,
    pub starting_y: i32,
    pub width: i32,
    pub height: i32,
    pub obstacle: Option<(i32, i32)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Space {
    Open,
    Obstacle,
}

impl Map {
    pub fn new(input: &str, obstacle: Option<(i32, i32)>) -> Self {
        let mut x = 0;
        let mut y = 0;

        let mut data = vec![];

        let mut starting_x = 0;
        let mut starting_y = 0;
        let mut width = 0;

        for c in input.chars() {
            if let Some((obstacle_x, obstacle_y)) = obstacle {
                if x == obstacle_x && y == obstacle_y {
                    data.push(Space::Obstacle);
                    x += 1;
                    continue;
                }
            }

            match c {
                '.' => {
                    data.push(Space::Open);
                }
                '#' => data.push(Space::Obstacle),
                '^' => {
                    starting_x = x;
                    starting_y = y;

                    data.push(Space::Open);
                }
                '\n' => {
                    if width == 0 {
                        width = x;
                    }

                    x = 0;
                    y += 1;
                    continue;
                }
                _ => panic!("Invalid character in map"),
            }

            x += 1;
        }

        Map {
            data,
            starting_x,
            starting_y,
            width,
            height: y,
            obstacle,
        }
    }

    pub fn get_position(&self, x: i32, y: i32) -> Option<Space> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        let index = (y * self.width) + x;
        self.data.get(index as usize).copied()
    }
}
