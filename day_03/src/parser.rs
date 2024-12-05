use std::{str::Chars, vec};

#[derive(Debug, Clone)]
pub enum Node {
    Do,
    Dont,
    Mul(i32, i32),
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    output: Vec<Node>,
    chars: Chars<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            output: vec![],
            chars: src.chars().to_owned(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        while let Some(c) = self.chars.next() {
            if c == 'm' {
                self.parse_mul_call();
                continue;
            }

            if c == 'd' {
                self.parse_conditional();
                continue;
            }
        }

        self.output.clone()
    }

    fn parse_conditional(&mut self) {
        let Some('o') = self.chars.next() else {
            return;
        };

        let Some(char) = self.chars.next() else {
            return;
        };

        if char == '(' {
            self.parse_do_call();
            return;
        }

        if char == 'n' {
            self.parse_dont_call();
        }
    }

    fn parse_do_call(&mut self) {
        if let Some(')') = self.chars.next() {
            self.output.push(Node::Do);
        }
    }

    fn parse_dont_call(&mut self) {
        for c in ['\'', 't', '(', ')'] {
            if let Some(current_char) = self.chars.next() {
                if current_char != c {
                    return;
                }
            }
        }

        self.output.push(Node::Dont);
    }

    fn parse_number_while_not(&mut self, end: char) -> Option<i32> {
        let mut output = String::new();

        for c in self.chars.by_ref() {
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
            let Some(current_char) = self.chars.next() else {
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

        self.output.push(Node::Mul(left, right))
    }
}
