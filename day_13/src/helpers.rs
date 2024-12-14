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
    fn test_get_slope_intercept_form() {
        let a = get_slope_intercept_form((0.0, 0.0), (1.0, 1.0));
        let b = get_slope_intercept_form((0.0, 2.0), (2.0, 0.0));
        let output = get_line_intersection(a, b);
        assert_eq!(output, Some((1.0, 1.0)));
    }
}
