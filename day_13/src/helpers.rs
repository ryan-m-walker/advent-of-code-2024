use core::panic;

use crate::{claw_machine::ClawMachine, Coord};

pub fn parse_input(input: &str, added: f64) -> Vec<ClawMachine> {
    let machines: Vec<_> = input.split("\n\n").collect();

    let mut output = vec![];

    for machine in machines {
        output.push(ClawMachine::new(machine, added));
    }

    output
}

pub fn parse_claw_machine(input: &str) -> [Coord; 3] {
    let lines: Vec<_> = input.lines().collect();

    let button_a = parse_button(lines[0]);
    let button_b = parse_button(lines[1]);
    let prize = parse_prize(lines[2]);

    [button_a, button_b, prize]
}

fn parse_button(line: &str) -> Coord {
    let moves = line.split(':').nth(1).unwrap().trim();
    let split: Vec<_> = moves.split(", ").collect();
    let x = split[0].replace("X+", "").trim().parse().unwrap();
    let y = split[1].replace("Y+", "").trim().parse().unwrap();

    (x, y)
}

fn parse_prize(line: &str) -> Coord {
    let split: Vec<_> = line.split(", ").collect();
    let x = split[0].replace("Prize: X=", "").trim().parse().unwrap();
    let y = split[1].replace("Y=", "").trim().parse().unwrap();

    (x, y)
}

pub fn find_intersection(a: Coord, b: Coord, c: Coord, d: Coord) -> Option<Coord> {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let (x3, y3) = c;
    let (x4, y4) = d;

    let a = (x4 - x3) * (y3 - y1) - (y4 - y3) * (x3 - x1);
    let b = (x4 - x3) * (y2 - y1) - (y4 - y3) * (x2 - x1);
    let c = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    let alpha = a / b;
    let beta = c / b;

    if b == 0.0 {
        panic!("Lines are parallel");
    }

    if a == 0.0 && b == 0.0 {
        panic!("Lines are collinear");
    }

    if !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&beta) {
        return None;
    }

    let px = x1 + alpha * (x2 - x1);
    let py = y1 + alpha * (y2 - y1);

    Some((px, py))
}

pub fn get_slope_intercept_form(a: Coord, b: Coord) -> (f64, f64) {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let m = (y2 - y1) / (x2 - x1);
    let b = y1 - m * x1;

    (m, b)
}

pub fn get_line_intersection(a: (f64, f64), b: (f64, f64)) -> Option<Coord> {
    let (m1, b1) = a;
    let (m2, b2) = b;

    if m1 == m2 {
        return None;
    }

    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = [
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ]
        .join("\n");

        parse_claw_machine(&input);
    }

    #[test]
    fn test_find_intersection() {
        let output = find_intersection((0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0));

        dbg!(output);
    }

    #[test]
    fn test_get_slope_intercept_form() {
        let a = get_slope_intercept_form((0.0, 0.0), (1.0, 1.0));

        let b = get_slope_intercept_form((0.0, 2.0), (2.0, 0.0));

        let output = get_line_intersection(a, b);

        dbg!(output);
    }
}
