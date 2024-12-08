use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub frequencies: HashMap<String, Vec<(i32, i32)>>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut frequencies = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;

        for c in input.chars() {
            if c == '\n' {
                y += 1;
                x = 0;

                if width == 0 {
                    width = x;
                }

                continue;
            }

            x += 1;

            if c == '.' {
                continue;
            }

            let frequency = frequencies.entry(c.to_string()).or_insert(Vec::new());
            frequency.push((x, y));
        }

        Map {
            width,
            height: y + 1,
            frequencies,
        }
    }
}
