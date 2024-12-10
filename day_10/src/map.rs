use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct HikingMap {
    data: Vec<i32>,
    width: i32,
    height: i32,
    starting_positions: Vec<(i32, i32)>,
}

impl HikingMap {
    pub fn new(input: &str) -> Self {
        let mut map = vec![];
        let mut starting_positions = vec![];

        let mut x = 0;
        let mut y = 0;

        let mut width = 0;

        for c in input.chars() {
            if c == '\n' {
                if width == 0 {
                    width = x;
                }

                y += 1;
                x = 0;
                continue;
            }

            if c == '.' {
                map.push(-1);
                x += 1;
                continue;
            }

            if c == '0' {
                starting_positions.push((x, y));
            }

            if !c.is_numeric() {
                continue;
            }

            if let Ok(parsed) = c.to_string().parse() {
                map.push(parsed);
            };

            x += 1;
        }

        Self {
            data: map,
            width,
            height: y + 1,
            starting_positions,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let index = y * self.width + x;
        self.data.get(index as usize).copied()
    }

    fn get_next_moves(&self, height: i32, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = vec![];

        for d in DIRECTIONS.iter() {
            let nx = x + d.0;
            let ny = y + d.1;

            if let Some(nh) = self.get(nx, ny) {
                if nh == height + 1 {
                    neighbors.push((nx, ny));
                }
            }
        }

        neighbors
    }

    pub fn find_all_trailheads(&self) -> i32 {
        let mut count = 0;

        for start in &self.starting_positions {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            count += self.find_trailheads(0, start.0, start.1, &mut visited);
        }

        count
    }

    fn find_trailheads(
        &self,
        height: i32,
        x: i32,
        y: i32,
        visited: &mut HashSet<(i32, i32)>,
    ) -> i32 {
        let next_positions = self.get_next_moves(height, x, y);

        if height == 9 && self.get(x, y).is_some() {
            if visited.contains(&(x, y)) {
                return 0;
            }

            visited.insert((x, y));
            return 1;
        }

        let mut count = 0;

        for next in next_positions {
            count += self.find_trailheads(height + 1, next.0, next.1, visited);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hiking_map_test_input_1() {
        let input = [
            "...0...", "...1...", "...2...", "6543456", "7.....7", "8.....8", "9.....9",
        ]
        .join("\n");
        let map = HikingMap::new(&input);
        let output = map.find_all_trailheads();
        assert_eq!(output, 2);
    }

    #[test]
    fn hiking_map_test_input_2() {
        let input = [
            "..90..9", "...1.98", "...2..7", "6543456", "765.987", "876....", "987....",
        ]
        .join("\n");
        let map = HikingMap::new(&input);
        let output = map.find_all_trailheads();
        assert_eq!(output, 4);
    }

    #[test]
    fn hiking_map_test_input_3() {
        let input = [
            "10..9..", "2...8..", "3...7..", "4567654", "...8..3", "...9..2", ".....01",
        ]
        .join("\n");
        let map = HikingMap::new(&input);
        let output = map.find_all_trailheads();
        assert_eq!(output, 3);
    }
}
