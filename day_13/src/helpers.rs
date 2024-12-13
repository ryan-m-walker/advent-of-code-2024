use crate::{claw_machine::ClawMachine, Coord};

pub fn parse_input(input: &str) -> Vec<ClawMachine> {
    let machines: Vec<_> = input.split("\n\n").collect();

    let mut output = vec![];

    for machine in machines {
        output.push(ClawMachine::new(machine));
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
}
