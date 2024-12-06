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

type MasMapping = [(i32, i32); 3];

const MAS_MAPPINGS: [MasMapping; 4] = [
    [(-1, -1), (0, 0), (1, 1)],
    [(1, -1), (0, 0), (-1, 1)],
    [(1, 1), (0, 0), (-1, -1)],
    [(-1, 1), (0, 0), (1, -1)],
];

#[derive(Debug)]
pub struct Finder<'a> {
    input: &'a str,
    row: i32,
    col: i32,
    col_count: i32,
    row_count: i32,
}

impl<'a> Finder<'a> {
    pub fn new(input: &'a str) -> Self {
        Finder {
            input,
            row: 0,
            col: 0,
            col_count: input.find('\n').unwrap_or(0) as i32,
            row_count: input.lines().count() as i32,
        }
    }

    /// Find all instances of the word "XMAS" in the input.
    pub fn find_xmas_count(&mut self) -> i32 {
        self.reset();

        let mut count = 0;

        for c in self.input.chars() {
            if c == '\n' {
                self.increment_row();
                continue;
            }

            for direction in DIRECTIONS.iter() {
                if self.scan_for_word(direction, &['X', 'M', 'A', 'S']) {
                    count += 1;
                }
            }

            self.col += 1;
        }

        count
    }

    pub fn find_mas_x_count(&mut self) -> i32 {
        self.reset();

        let mut count = 0;

        for c in self.input.chars() {
            if c == '\n' {
                self.increment_row();
                continue;
            }

            if self.is_window_in_bounds() && self.scan_for_mas() {
                count += 1;
            }

            self.col += 1;
        }

        count
    }

    fn reset(&mut self) {
        self.row = 0;
        self.col = 0;
    }

    fn increment_row(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    fn is_window_in_bounds(&self) -> bool {
        let is_in_left_bound = self.col > 0;
        let is_in_right_bound = self.col + 1 < self.col_count;
        let is_in_top_bound = self.row > 0;
        let is_in_bottom_bound = self.row + 1 < self.row_count;

        is_in_left_bound && is_in_right_bound && is_in_top_bound && is_in_bottom_bound
    }

    fn scan_for_mas(&self) -> bool {
        let mut count = 0;

        'outer: for mapping in MAS_MAPPINGS.iter() {
            for (index, char) in ['M', 'A', 'S'].iter().enumerate() {
                let (x_offset, y_offset) = mapping[index];
                let x = self.col + x_offset;
                let y = self.row + y_offset;

                let Some(target) = self.get_char(x, y) else {
                    continue 'outer;
                };

                if target != *char {
                    continue 'outer;
                }
            }

            count += 1;
        }

        count == 2
    }

    fn scan_for_word(&self, direction: &Direction, word: &[char]) -> bool {
        for (i, char) in word.iter().enumerate() {
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
        let row_offset = y * (self.col_count + 1);
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

    // find_xmas_count tests

    #[test]
    fn find_xmas_count_simple_up() {
        let input = "S\nA\nM\nX\n";
        let mut finder = Finder::new(input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_up_right() {
        let input = ["...S", "..A.", ".M..", "X..."].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_right() {
        let input = "XMAS\n";
        let mut finder = Finder::new(input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_down_right() {
        let input = ["X...", ".M..", "..A.", "...S"].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_down() {
        let input = "X\nM\nA\nS\n";
        let mut finder = Finder::new(input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_down_left() {
        let input = ["...X", "..M.", ".A..", "S..."].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_left() {
        let input = "SAMX\n";
        let mut finder = Finder::new(input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    #[test]
    fn find_xmas_count_simple_up_left() {
        let input = ["S...", ".A..", "..M.", "...X"].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find_xmas_count();
        assert_eq!(output, 1);
    }

    // find_mas_x_count tests

    #[test]
    fn find_mas_x_count_simple_up_right_down_left() {
        let input = ["M.M", ".A.", "S.S"].join("\n");
        let mut finder = Finder::new(&input);
        let output = finder.find_mas_x_count();
        assert_eq!(output, 1);
    }
}
