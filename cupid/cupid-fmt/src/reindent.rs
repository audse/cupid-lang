use std::{iter::Peekable, str::Chars};

pub trait Reindent {
    fn reindent(self, indent_size: usize) -> String;
}

fn empty_string(length: impl Into<usize>) -> String {
    let mut s = vec![];
    s.resize_with(length.into(), || " ");
    s.join("")
}

impl Reindent for String {
    fn reindent(self, indent_size: usize) -> String {
        let mut new_string = String::new();
        let mut stack = vec![];
        let mut chars = self.chars().peekable();

        while let Some(c) = chars.next() {
            let peek = |chars: &mut Peekable<Chars>| chars.peek().copied().unwrap_or_default();

            match c {
                '[' | '{' | '(' => {
                    stack.push(c);
                    new_string.push(c);
                }
                ']' | '}' | ')' => {
                    stack.pop();
                    new_string.push(c);
                }
                '\n' if [']', '}', ')'].contains(&peek(&mut chars)) => {
                    let next = chars.next().unwrap();
                    new_string.push(c);
                    stack.pop();
                    new_string.push_str(&empty_string(stack.len() * indent_size));
                    new_string.push(next);
                }
                '\n' => {
                    new_string.push(c);
                    new_string.push_str(&empty_string(stack.len() * indent_size))
                }
                _ => new_string.push(c),
            }
        }
        new_string
    }
}

pub trait Multiline {
    fn multiline(self, max_length: usize) -> String;
}

fn close_bracket(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("Not a bracket: {}", c),
    }
}

pub struct BidirectionalIterator<T> {
    curr: usize,
    items: Vec<T>,
}

impl<T> BidirectionalIterator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            curr: 0,
            items: items.into(),
        }
    }
}

impl<T: Copy> Iterator for BidirectionalIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        self.items.get(self.curr).copied()
    }
}

impl<T: Copy> BidirectionalIterator<T> {
    pub fn peek(&mut self) -> Option<T> {
        self.peek_by(1)
    }
    pub fn peek_by(&mut self, amount: usize) -> Option<T> {
        match (self.curr + amount) < self.items.len() {
            true => self.items.get(self.curr + amount).copied(),
            false => None,
        }
    }
    pub fn move_by(&mut self, amount: usize) -> Option<T> {
        self.curr += amount;
        self.items.get(self.curr).copied()
    }
    pub fn move_to(&mut self, pos: usize) -> Option<T> {
        self.curr = pos;
        self.items.get(self.curr).copied()
    }
}

impl From<Chars<'_>> for BidirectionalIterator<char> {
    fn from(value: Chars) -> Self {
        Self::new(value.collect())
    }
}

impl Multiline for String {
    fn multiline(self, max_length: usize) -> String {
        let mut string = String::new();

        let mut chars: BidirectionalIterator<char> = self.chars().into();
        let mut should_linebreak_stack = vec![];

        while let Some(c) = chars.next() {
            let skip_whitespace = |chars: &mut BidirectionalIterator<char>| {
                while [' ', '\t'].contains(&chars.peek().unwrap_or_default()) {
                    chars.next();
                }
            };
            match c {
                ',' => {
                    string.push(c);
                    match should_linebreak_stack.last() {
                        Some(true) => {
                            string.push('\n');
                            skip_whitespace(&mut chars);
                        }
                        _ => (),
                    }
                }
                '[' | '{' | '(' => {
                    string.push(c);
                    let matching = close_bracket(c);
                    let mut length = 0;
                    loop {
                        length += 1;
                        match chars.peek_by(length) {
                            Some(n) if n == matching => break,
                            _ => (),
                        }
                    }
                    should_linebreak_stack.push(length > max_length);
                    if length > max_length {
                        string.push('\n');
                        skip_whitespace(&mut chars);
                    } else if c == '{' {
                        skip_whitespace(&mut chars);
                        string.push(' ');
                    }
                }
                ']' | '}' | ')' => {
                    match should_linebreak_stack.pop() {
                        Some(true) => string.push('\n'),
                        _ => {}
                    }
                    string.push(c);
                }
                _ => {
                    string.push(c);
                }
            }
        }
        string
    }
}
