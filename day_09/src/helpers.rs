#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {
    File(i64),
    Space,
}

pub fn parse_input(input: &str) -> Vec<Block> {
    let mut output = Vec::new();
    let mut id = 0;

    for (i, c) in input.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }

        let size: i64 = c.to_string().parse().unwrap();

        let block = if i % 2 == 0 {
            let current_id = id;
            id += 1;
            Block::File(current_id)
        } else {
            Block::Space
        };

        for _ in 0..size {
            output.push(block);
        }
    }

    output
}

pub fn blocks_to_string(block: &[Block]) -> String {
    let mut output = String::new();

    for b in block {
        match b {
            Block::File(id) => output.push_str(&id.to_string()),
            Block::Space => output.push('.'),
        }
    }

    output
}

pub fn print(blocks: &[Block]) {
    println!("{}", blocks_to_string(blocks));
}

pub fn print_with_highlights(blocks: &[Block], head_hl: usize, tail_hl: usize) {
    let string = blocks_to_string(blocks);

    for (i, c) in string.chars().enumerate() {
        if i == head_hl {
            print!("\x1b[0;31m{}\x1b[0m", c);
        } else if i == tail_hl {
            print!("\x1b[0;32m{}\x1b[0m", c);
        } else {
            print!("{}", c);
        }
    }

    println!();
}

pub fn format_disk(blocks: &mut [Block]) {
    let mut head_ptr = 0;
    let mut tail_ptr = blocks.len() - 1;

    loop {
        if head_ptr >= tail_ptr {
            break;
        }

        if blocks[tail_ptr] == Block::Space {
            tail_ptr -= 1;
            continue;
        }

        loop {
            let head = blocks[head_ptr];

            match head {
                Block::File(_) => {
                    head_ptr += 1;
                    continue;
                }
                Block::Space => {
                    blocks.swap(head_ptr, tail_ptr);
                    break;
                }
            }
        }

        tail_ptr -= 1;

        if head_ptr.abs_diff(tail_ptr) <= 1 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_input_test() {
        let input = fs::read_to_string("./example_input_2.txt").unwrap();
        let blocks = parse_input(&input);
        let output = blocks_to_string(&blocks);
        assert_eq!(output, String::from("0..111....22222"));
    }

    #[test]
    fn parse_input_test_2() {
        let input = "2333133121414131402";
        let blocks = parse_input(input);
        let output = blocks_to_string(&blocks);
        assert_eq!(
            output,
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );
    }

    #[test]
    fn print_test() {
        let input = "12345";
        let blocks = parse_input(input);
        let output = blocks_to_string(&blocks);
        assert_eq!(output, String::from("0..111....22222"));
    }

    #[test]
    fn format_disk_test() {
        let input = "12345";
        let mut blocks = parse_input(input);
        format_disk(&mut blocks);
        let output = blocks_to_string(&blocks);
        assert_eq!(output, String::from("022111222......"));
    }

    #[test]
    fn format_disk_test_2() {
        let input = "2333133121414131402";
        let mut blocks = parse_input(input);
        format_disk(&mut blocks);
        let output = blocks_to_string(&blocks);
        assert_eq!(
            output,
            String::from("0099811188827773336446555566..............")
        );
    }
}
