use crate::{helpers::parse_claw_machine, Coord};

#[derive(Debug, PartialEq)]
pub struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
    pos: Coord,
}

impl ClawMachine {
    pub fn new(input: &str) -> Self {
        let parsed = parse_claw_machine(input);

        Self {
            button_a: parsed[0],
            button_b: parsed[1],
            prize: parsed[2],
            pos: (0, 0),
        }
    }

    pub fn brute_force(&self) -> Option<(i32, i32)> {
        let mut n = 0;

        loop {
            if n > self.prize.0 {
                return None;
            }

            if let Some((a_presses, b_presses)) = self.run(n) {
                return Some((a_presses, b_presses));
            }

            n += 1;
        }
    }

    fn run(&self, b_presses: i32) -> Option<(i32, i32)> {
        let mut pos = (0, 0);

        let mut button = self.button_b;
        let mut i = 0;

        while pos != self.prize {
            if pos.0 > self.prize.0 || pos.1 > self.prize.1 {
                return None;
            }

            if i == b_presses {
                button = self.button_a;
            }

            pos = (pos.0 + button.0, pos.1 + button.1);
            i += 1;
        }

        Some((i - b_presses, b_presses))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let input = [
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ]
        .join("\n");

        let machine = ClawMachine::new(&input);

        let output = machine.brute_force();
    }
}
