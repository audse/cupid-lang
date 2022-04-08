use crate::{Rule, Tokenizer};
use std::collections::HashMap;
mod grammar;
pub use grammar::Parser as GParser;
mod cupid;
pub use cupid::Parser as CParser;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<String>,
}

pub struct Parser {
    pub index: usize,
    pub tokens: Vec<String>,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        tokenizer.scan();
        Self {
            index: 0,
            tokens: tokenizer.tokens,
        }
    }
    fn mark(&self) -> usize {
        self.index
    }
    fn reset(&mut self, pos: usize) {
        self.index = pos;
    }
    fn current(&self) -> String {
        self.tokens[self.index].clone()
    }
    fn expect(&mut self, string: &str) -> Option<String> {
        let next = self.current();
        if next == string.to_string() {
            self.index += 1;
            return Some(next);
        }
        return None;
    }
    fn expect_number(&mut self) -> Option<String> {
        let next = self.current();
        let chars: Vec<char> = next.chars().collect();
        for char in chars {
            if !char.is_digit(10) {
                return None;
            }
        }
        self.index += 1;
        return Some(next);
    }
    fn expect_word(&mut self) -> Option<String> {
        let next = self.current();
        let chars: Vec<char> = next.chars().collect();
        for char in chars {
            if !char.is_alphabetic() {
                return None;
            }
        }
        self.index += 1;
        return Some(next);
    }
    fn expect_string(&mut self) -> Option<String> {
        let next = self.current();
        if &next == "\"" || &next == "'" {
            //"
            while self.current() != next {
                self.index += 1;
            }
            self.index += 1;
            return Some(next.clone());
        }
        return None;
    }

    pub fn _term(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(function_call0) = self._function_call() {
            children.push(function_call0);
            return Some(Node {
                name: String::from("term"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(atom0) = self._atom() {
            children.push(atom0);
            return Some(Node {
                name: String::from("term"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _function_call(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(identifier0) = self._identifier() {
            children.push(identifier0);

            if let Some(_token1) = self.expect("(") {
                tokens.push(_token1);

                if let Some(args23) = self._args() {
                    children.push(args23);
                }

                if let Some(_token3) = self.expect(")") {
                    tokens.push(_token3);
                    return Some(Node {
                        name: String::from("function_call"),
                        tokens,
                        children,
                    });
                }
            }
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _args(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        while let Some(arg_item01) = self._arg_item() {
            children.push(arg_item01);
        }

        if let Some(identifier1) = self._identifier() {
            children.push(identifier1);
            return Some(Node {
                name: String::from("args"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _arg_item(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(identifier0) = self._identifier() {
            children.push(identifier0);

            if let Some(_token1) = self.expect(",") {
                tokens.push(_token1);
                return Some(Node {
                    name: String::from("arg_item"),
                    tokens,
                    children,
                });
            }
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _atom(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(_token0) = self.expect("true") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(_token0) = self.expect("false") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(_token0) = self.expect("none") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(string0) = self._string() {
            children.push(string0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(decimal0) = self._decimal() {
            children.push(decimal0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(number0) = self._number() {
            children.push(number0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        if let Some(identifier0) = self._identifier() {
            children.push(identifier0);
            return Some(Node {
                name: String::from("atom"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _identifier(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(word0) = self.expect_word() {
            tokens.push(word0);
            return Some(Node {
                name: String::from("identifier"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _string(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(string0) = self.expect_string() {
            tokens.push(string0);
            return Some(Node {
                name: String::from("string"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _decimal(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(number0) = self.expect_number() {
            tokens.push(number0);

            if let Some(_token1) = self.expect(".") {
                tokens.push(_token1);

                if let Some(number2) = self.expect_number() {
                    tokens.push(number2);
                    return Some(Node {
                        name: String::from("decimal"),
                        tokens,
                        children,
                    });
                }
            }
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }

    pub fn _number(&mut self) -> Option<Node> {
        let pos = self.mark();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];

        if let Some(number0) = self.expect_number() {
            tokens.push(number0);
            return Some(Node {
                name: String::from("number"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.reset(pos);

        return None;
    }
}
