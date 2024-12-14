use crate::{
    helpers::{get_line_intersection, get_slope_intercept_form, parse_claw_machine},
    Coord,
};

#[derive(Debug, PartialEq)]
pub struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
    pos: Coord,
}

impl ClawMachine {
    pub fn new(input: &str, added: f64) -> Self {
        let parsed = parse_claw_machine(input);

        Self {
            button_a: parsed[0],
            button_b: parsed[1],
            prize: (parsed[2].0 + added, parsed[2].1 + added),
            pos: (0.0, 0.0),
        }
    }

    pub fn compute(&self) -> Option<Coord> {
        let line_a = get_slope_intercept_form((0.0, 0.0), self.button_a);

        let line_b = get_slope_intercept_form(
            self.prize,
            (
                self.prize.0 + self.button_b.0,
                self.prize.1 + self.button_b.1,
            ),
        );

        if let Some(intersection) = get_line_intersection(line_a, line_b) {
            let intersection = (intersection.0.round(), intersection.1.round());

            let a_x_is_divisible = intersection.0 % self.button_a.0 == 0.0;
            let a_y_is_divisible = intersection.1 % self.button_a.1 == 0.0;
            let a_is_divisible = a_x_is_divisible && a_y_is_divisible;

            let b_x_is_divisible = (self.prize.0 - intersection.0) % self.button_b.0 == 0.0;
            let b_y_is_divisible = (self.prize.1 - intersection.1) % self.button_b.1 == 0.0;
            let b_is_divisible = b_x_is_divisible && b_y_is_divisible;

            if a_is_divisible && b_is_divisible {
                let a_presses = intersection.0 / self.button_a.0;
                let b_presses = (self.prize.0 - intersection.0) / self.button_b.0;

                return Some((a_presses, b_presses));
            }
        }

        None
    }

    pub fn brute_force(&self) -> Option<Coord> {
        let mut n: f64 = 0.0;

        loop {
            if n > self.prize.0 {
                return None;
            }

            if let Some((a_presses, b_presses)) = self.run(n) {
                return Some((a_presses, b_presses));
            }

            n += 1.0;
        }
    }

    fn run(&self, b_presses: f64) -> Option<Coord> {
        let mut pos = (0.0, 0.0);

        let mut button = self.button_b;
        let mut i = 0.0;

        while pos != self.prize {
            if pos.0 > self.prize.0 || pos.1 > self.prize.1 {
                return None;
            }

            if i == b_presses {
                button = self.button_a;
            }

            pos = (pos.0 + button.0, pos.1 + button.1);
            i += 1.0;
        }

        Some((i - b_presses, b_presses))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn new_test() {
    //     let input = [
    //         "Button A: X+94, Y+34",
    //         "Button B: X+22, Y+67",
    //         "Prize: X=8400, Y=5400",
    //     ]
    //     .join("\n");
    //
    //     let machine = ClawMachine::new(&input);
    //
    //     let output = machine.brute_force();
    // }

    // #[test]
    // fn new_test_2() {
    //     let input = [
    //         "Button A: X+3, Y+1",
    //         "Button B: X+1, Y+3",
    //         "Prize: X=8, Y=8",
    //     ]
    //     .join("\n");
    //
    //     let machine = ClawMachine::new(&input);
    //
    //     let output = machine.compute();
    //     dbg!(output);
    // }

    #[test]
    fn new_test_3() {
        let input = [
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
        ]
        .join("\n");

        let machine = ClawMachine::new(&input);

        let output = machine.compute();
        dbg!(output);
    }
}
