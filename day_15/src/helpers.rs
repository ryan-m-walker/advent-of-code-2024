#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_directions(input: &str) -> Vec<Direction> {
    let mut directions = Vec::new();

    for char in input.chars() {
        match char {
            '^' => directions.push(Direction::Up),
            'v' => directions.push(Direction::Down),
            '<' => directions.push(Direction::Left),
            '>' => directions.push(Direction::Right),
            _ => {}
        }
    }

    directions
}
