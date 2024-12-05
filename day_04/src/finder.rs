type Direction = (i32, i32);

const DIRECTIONS: [Direction; 8] = [
    (0, -1),  // up
    (-1, 1),  // up-right
    (1, 0),   // right
    (1, 1),   // down-right
    (0, 1),   // down
    (1, -1),  // down-left
    (-1, 0),  // left
    (-1, -1), // up-left
];

#[derive(Debug)]
pub struct Finder<'a> {
    input: &'a str,
    count: i32,
    row: i32,
    col: i32,
    col_size: i32,
}

impl<'a> Finder<'a> {
    pub fn new(input: &'a str) -> Self {
        Finder {
            input,
            count: 0,
            row: 0,
            col: 0,
            col_size: input.find('\n').unwrap_or(0) as i32,
        }
    }

    pub fn find(&mut self) -> i32 {
        for c in self.input.chars() {
            if c == '\n' {
                self.row += 1;
                self.col = 0;
                continue;
            }

            for direction in DIRECTIONS.iter() {
                if self.scan(direction) {
                    self.count += 1;
                }
            }

            self.col += 1;
        }

        self.count
    }

    fn scan(&self, direction: &Direction) -> bool {
        for (i, char) in ['X', 'M', 'A', 'S'].iter().enumerate() {
            let x = self.col + (i as i32 * direction.0);
            let y = self.row + (i as i32 * direction.1);

            let Some(target) = self.get_char(x, y) else {
                return false;
            };

            if target != *char {
                return false;
            }
        }

        true
    }

    fn get_char(&self, x: i32, y: i32) -> Option<char> {
        let row_offset = y * (self.col_size + 1);
        let index_start = (row_offset + x) as usize;
        let index_end = (row_offset + x + 1) as usize;

        self.input
            .get(index_start..index_end)
            .and_then(|s| s.chars().next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finder_test_simple_up() {
        let input = "S\nA\nM\nX\n";
        let mut finder = Finder::new(input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_up_right() {
        let input = ["...S", "..A.", ".M..", "X..."].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_right() {
        let input = "XMAS\n";
        let mut finder = Finder::new(input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_down_right() {
        let input = ["X...", ".M..", "..A.", "...S"].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_down() {
        let input = "X\nM\nA\nS\n";
        let mut finder = Finder::new(input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_down_left() {
        let input = ["...X", "..M.", ".A..", "S..."].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_left() {
        let input = "SAMX\n";
        let mut finder = Finder::new(input);
        let output = finder.find();
        assert_eq!(output, 1);
    }

    #[test]
    fn finder_test_simple_up_left() {
        let input = ["S...", ".A..", "..M.", "...X"].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find();
        assert_eq!(output, 1);
    }
}
