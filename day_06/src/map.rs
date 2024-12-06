#[derive(Debug)]
pub struct Map {
    data: Vec<Space>,
    pub starting_x: i32,
    pub starting_y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Space {
    Open,
    Obstacle,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;

        let mut data = vec![];

        let mut starting_x = 0;
        let mut starting_y = 0;
        let mut width = 0;

        for c in input.chars() {
            match c {
                '.' => {
                    x += 1;
                    data.push(Space::Open);
                }
                '#' => {
                    x += 1;
                    data.push(Space::Obstacle)
                }
                '^' => {
                    starting_x = x;
                    starting_y = y;

                    x += 1;
                    data.push(Space::Open);
                }
                '\n' => {
                    if width == 0 {
                        width = x;
                    }

                    x = 0;
                    y += 1;
                }
                _ => panic!("Invalid character in map"),
            }
        }

        Map {
            data,
            starting_x,
            starting_y,
            width,
            height: y,
        }
    }

    pub fn get_position(&self, x: i32, y: i32) -> Option<Space> {
        let index = (y * self.width) + x;
        self.data.get(index as usize).copied()
    }
}
