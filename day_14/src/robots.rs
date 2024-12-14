use crate::robot::{Quad, Robot};

pub struct Robots {
    pub robots: Vec<Robot>,
    pub dimensions: (i32, i32),
}

impl Robots {
    pub fn new(input: &str, dimensions: (i32, i32)) -> Self {
        let robots = input
            .lines()
            .map(|line| Robot::new(line, dimensions))
            .collect();

        Self { robots, dimensions }
    }

    pub fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.step();
        }
    }

    pub fn step_n_times(&mut self, n: i32) {
        for _ in 0..n {
            self.step();
        }
    }

    pub fn quads(&self) -> [i32; 4] {
        let mut tl = 0;
        let mut tr = 0;
        let mut bl = 0;
        let mut br = 0;

        for robot in self.robots.iter() {
            match robot.quad() {
                Some(Quad::TopLeft) => tl += 1,
                Some(Quad::TopRight) => tr += 1,
                Some(Quad::BottomLeft) => bl += 1,
                Some(Quad::BottomRight) => br += 1,
                None => {}
            }
        }

        [tl, tr, bl, br]
    }

    pub fn maybe_tree(&self) -> bool {
        for quad in self.quads().iter() {
            let three_quarters = self.robots.len() as i32 / 2;
            if quad > &three_quarters {
                return true;
            }
        }

        false
    }

    pub fn print(&self) {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let mut found = false;

                for robot in self.robots.iter() {
                    if robot.pos == (x, y) {
                        print!("#");
                        found = true;
                        break;
                    }
                }

                if !found {
                    print!(".");
                }
            }

            println!();
        }

        println!();
    }
}
