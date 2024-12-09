use crate::helpers::{print, Block};

#[derive(Debug)]
pub struct Memory<'a> {
    pub blocks: &'a mut Vec<Block>,
}

impl<'a> Memory<'a> {
    pub fn new(blocks: &'a mut Vec<Block>) -> Self {
        print(blocks);
        Self { blocks }
    }

    pub fn compress(&mut self) {
        let mut file_from = self.blocks.len() - 1;

        loop {
            let (file_start, file_end, file_id) = self.get_file(file_from);
            let size = file_end.abs_diff(file_start) + 1;

            let space = self.find_available_space(size, file_start);

            file_from = file_start - 1;

            if let Some((space_start, space_end)) = space {
                for i in space_start..=space_end {
                    self.blocks[i] = Block::File(file_id);
                }

                for i in file_start..=file_end {
                    self.blocks[i] = Block::Space;
                }
            }

            if file_id == 0 {
                break;
            }
        }
    }

    pub fn find_available_space(&self, size: usize, until: usize) -> Option<(usize, usize)> {
        let mut start = 0;
        let mut end = 0;

        let mut in_space = false;

        for (i, block) in self.blocks.iter().enumerate() {
            if i > until {
                break;
            }

            match block {
                Block::Space => {
                    if !in_space {
                        start = i;
                        in_space = true;
                    }

                    end = i;

                    let space_size = end.abs_diff(start) + 1;

                    if space_size >= size {
                        return Some((start, end));
                    }
                }
                Block::File(_) => {
                    if in_space {
                        let space_size = end - start + 1;

                        if space_size >= size {
                            return Some((start, end));
                        }

                        in_space = false;
                    }
                }
            }
        }

        None
    }

    pub fn get_next_space(&self, from: usize) -> Option<(usize, usize)> {
        let mut start = 0;
        let mut end = 0;

        let mut found = false;

        for (i, block) in self.blocks.iter().enumerate() {
            if i < from {
                continue;
            }

            if i >= self.blocks.len() {
                break;
            }

            match block {
                Block::Space => {
                    if !found {
                        start = i;
                        found = true;
                    }
                    end = i;
                }
                Block::File(_) => {
                    if found {
                        break;
                    }
                }
            }
        }

        if found {
            return Some((start, end));
        }

        None
    }

    pub fn find_space(&self, size: usize) -> Option<(usize, usize)> {
        let mut window_start = 0;

        loop {
            let space = self.get_next_space(window_start);

            let Some((start, end)) = space else {
                break;
            };

            // println!("start: {}, end: {}", start, end);

            if end - start >= size {
                return Some((start, end));
            }

            println!("{}", end);
            window_start = end + 1;

            if window_start >= self.blocks.len() {
                println!("No space found");
                break;
            }
        }

        None
    }

    pub fn get_file(&self, from: usize) -> (usize, usize, i64) {
        let mut start = from;
        let mut end = from;

        loop {
            let end_block = self.blocks.get(end);

            let Some(Block::File(id)) = end_block else {
                end -= 1;
                start = end;
                continue;
            };

            loop {
                if start == 0 {
                    return (start + 1, end, *id);
                }

                let start_block = self.blocks[start];

                if start_block != Block::File(*id) {
                    return (start + 1, end, *id);
                }

                start -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::helpers::{blocks_to_string, parse_input};

    use super::*;

    // #[test]
    // fn memory_test() {
    //     let input = "12345";
    //     let blocks = parse_input(input);
    //     let memory = Memory::new(&blocks);
    //
    //     let first = memory.get_next_space(0);
    //     assert_eq!(first, Some((1, 2)));
    //
    //     let second = memory.get_next_space(first.unwrap().1 + 1);
    //     assert_eq!(second, Some((6, 9)));
    // }
    //
    // #[test]
    // fn find_space() {
    //     let input = "12345";
    //     let blocks = parse_input(input);
    //     let memory = Memory::new(&blocks);
    //
    //     let output = memory.find_space(3);
    //     assert_eq!(output, Some((6, 9)));
    // }
    //
    // #[test]
    // fn find_next_file() {
    //     let input = "12345";
    //     let blocks = parse_input(input);
    //     let memory = Memory::new(&blocks);
    //
    //     let first = memory.get_file(blocks.len());
    //     assert_eq!(first, (10, 14, 2));
    //     let second = memory.get_file(first.0 - 1);
    //     assert_eq!(second, (3, 5, 1));
    //     let third = memory.get_file(second.0 - 1);
    //     assert_eq!(third, (0, 0, 0));
    // }

    // #[test]
    // fn compress() {
    // let input = "111";
    // let mut blocks = parse_input(input);
    // let mut memory = Memory::new(&mut blocks);
    // memory.compress();
    // assert_eq!(blocks_to_string(memory.blocks), "01.");
    // }

    #[test]
    fn compress_example_input() {
        let input = "2333133121414131402";
        let mut blocks = parse_input(input);
        let mut memory = Memory::new(&mut blocks);
        memory.compress();

        assert_eq!(
            blocks_to_string(memory.blocks),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }
}
