use std::{mem, str::Chars, vec};

#[derive(Debug, Clone)]
pub enum Node {
    Do,
    Dont,
    Mul(Mul),
}

#[derive(Debug, Clone)]
pub struct Mul {
    pub left: i32,
    pub right: i32,
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    ptr: usize,
    output: Vec<Node>,
    chars: Chars<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            ptr: 0,
            output: vec![],
            chars: src.chars().to_owned(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        while let Some(c) = self.next() {
            dbg!(c);
            if c == 'm' {
                dbg!("mul");
                self.parse_mul_call();
                continue;
            }

            if c == 'd' {
                dbg!("cond");
                self.parse_conditional();
                continue;
            }
        }

        self.output.clone()
    }

    fn next(&mut self) -> Option<char> {
        let char = self.chars.nth(self.ptr);
        self.ptr += 1;
        char
    }

    fn parse_conditional(&mut self) {
        let Some('o') = self.next() else {
            return;
        };

        let Some(char) = self.next() else {
            return;
        };

        if char == '(' {
            self.parse_do_call();
            return;
        }

        if char == 'n' {
            self.parse_dont_call();
            return;
        }
    }

    fn parse_do_call(&mut self) {
        if let Some(')') = self.next() {
            self.output.push(Node::Do);
        }
    }

    fn parse_dont_call(&mut self) {
        for c in ['o', 'n', '\'', 't', '(', ')'] {
            if let Some(current_char) = self.next() {
                if current_char != c {
                    return;
                }
            }
        }

        self.output.push(Node::Dont);
    }

    fn parse_number_while_not(&mut self, end: char) -> Option<i32> {
        let mut output = String::new();

        while let Some(c) = self.next() {
            if c == end {
                if let Ok(value) = output.parse() {
                    return Some(value);
                }
            }

            if !c.is_numeric() {
                return None;
            }

            output.push(c);
        }

        None
    }

    fn parse_mul_call(&mut self) {
        for c in ['u', 'l', '('] {
            let Some(current_char) = self.next() else {
                return;
            };

            if current_char != c {
                return;
            }
        }

        let Some(left) = self.parse_number_while_not(',') else {
            return;
        };

        let Some(right) = self.parse_number_while_not(')') else {
            return;
        };

        self.output.push(Node::Mul(Mul { left, right }))
    }
}
