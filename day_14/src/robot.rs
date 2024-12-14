#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Quad {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    pub pos: (i32, i32),
    pub vel: (i32, i32),
    pub dimensions: (i32, i32),
}

impl Robot {
    pub fn new(input: &str, dimensions: (i32, i32)) -> Self {
        let split: Vec<_> = input.split(" ").collect();

        let pos = split[0].trim().replace("p=", "");
        let pos: Vec<_> = pos.split(",").collect();
        let pos_x = pos[0].parse().unwrap();
        let pos_y = pos[1].parse().unwrap();

        let vel = split[1].trim().replace("v=", "");
        let vel: Vec<_> = vel.split(",").collect();
        let vel_x = vel[0].parse().unwrap();
        let vel_y = vel[1].parse().unwrap();

        Self {
            pos: (pos_x, pos_y),
            vel: (vel_x, vel_y),
            dimensions,
        }
    }

    pub fn step(&mut self) {
        let mut new_x = (self.pos.0 + self.vel.0) % self.dimensions.0;
        let mut new_y = (self.pos.1 + self.vel.1) % self.dimensions.1;

        if new_x < 0 {
            new_x += self.dimensions.0;
        }

        if new_y < 0 {
            new_y += self.dimensions.1;
        }

        self.pos = (new_x, new_y);
    }

    pub fn quad(&self) -> Option<Quad> {
        let (x, y) = self.pos;

        let vertical_middle = self.dimensions.0 / 2;
        let horizontal_middle = self.dimensions.1 / 2;

        if x < vertical_middle && y < horizontal_middle {
            return Some(Quad::TopLeft);
        };

        if x > vertical_middle && y < horizontal_middle {
            return Some(Quad::TopRight);
        };

        if x > vertical_middle && y > horizontal_middle {
            return Some(Quad::BottomRight);
        };

        if x < vertical_middle && y > horizontal_middle {
            return Some(Quad::BottomLeft);
        };

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let robot = Robot::new("p=0,4 v=3,-3", (11, 7));
        assert_eq!(
            robot,
            Robot {
                pos: (0, 4),
                vel: (3, -3),
                dimensions: (11, 7)
            }
        );
    }

    #[test]
    fn step_test() {
        let mut robot = Robot::new("p=2,4 v=2,-3", (11, 7));

        assert_eq!(robot.pos, (2, 4));
        assert_eq!(robot.quad(), Some(Quad::BottomLeft));

        let expected = [
            (4, 1, Some(Quad::TopLeft)),
            (6, 5, Some(Quad::BottomRight)),
            (8, 2, Some(Quad::TopRight)),
            (10, 6, Some(Quad::BottomRight)),
            (1, 3, None),
        ];

        for (x, y, quad) in expected.iter() {
            robot.step();
            assert_eq!(robot.pos, (*x, *y));
            assert_eq!(robot.quad(), *quad);
        }
    }
}
