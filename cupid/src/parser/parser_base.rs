#![allow(clippy::all)]
use crate::{
    is_identifier, is_number, is_string, is_uppercase, BiDirectionalIterator, Tokenizer, Token,
};
use std::collections::HashMap;
use std::hash::Hash;

macro_rules! use_item {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        if let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                // move returned node's children to current node
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        } else {
            break;
        }
    }};
}

macro_rules! use_optional {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        if let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        }
    }};
}

macro_rules! use_repeat {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        while let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        }
    }};
}

macro_rules! use_negative_lookahead {
    ($parser:expr, $index:expr, $method:expr) => {{
        $parser.tokens.goto($index);
        if let Some((_val, _pass_through)) = $method {
            break;
        }
    }};
}

macro_rules! use_positive_lookahead {
    ($parser:expr, $index:expr, $method:expr) => {{
        $parser.tokens.goto($index);
        if let Some((_val, _pass_through)) = $method {
        } else {
            break;
        }
    }};
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<Token>,
}

#[derive(PartialEq, Eq)]
pub struct Parser {
    pub tokens: BiDirectionalIterator<Token>,
    pub index: usize,
    pub memos: Memos,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParseResult(Option<(Node, bool)>, usize);

#[derive(PartialEq, Eq)]
pub struct Memos {
    memos: HashMap<(usize, String), ParseResult>,
}

impl Memos {
    fn has_memo(&self, pos: usize, func: String) -> bool {
        self.memos.contains_key(&(pos, func))
    }
    fn get_memo(&mut self, pos: usize, func: String) -> Option<&ParseResult> {
        self.memos.get(&(pos, func))
    }
    fn add_memo(&mut self, pos: usize, func: String, result: ParseResult) -> ParseResult {
        self.memos.insert((pos, func.clone()), result);
        self.memos.get(&(pos, func)).unwrap().clone()
    }
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Tokenizer::new(source.as_str());
        tokenizer.scan();
        Self {
            index: 0,
            tokens: BiDirectionalIterator::new(tokenizer.tokens),
            memos: Memos {
                memos: HashMap::new(),
            },
        }
    }
    fn expect(&mut self, rule_name: String) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect".to_string(), Some(rule_name.clone())) {
            return Some(memo);
        }
        if let Some(token) = self.tokens.peek(0) {
            if token.source == rule_name {
                return Some((node_from_token(self.tokens.next().unwrap(), rule_name), true));
            }
        }
        None
    }
    fn expect_one(&mut self, rule_names: Vec<String>) -> Option<(Node, bool)> {
        if let Some(memo) =
            self.memoize("expect_one".to_string(), Some(format!("{:?}", rule_names)))
        {
            return Some(memo);
        }
        for rule_name in rule_names {
            if let Some(next) = self.expect(rule_name) {
                return Some(next);
            }
        }
        None
    }
    fn expect_constant(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_constant".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_uppercase(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "constant".to_string()), true));
            }
        }
        None
    }
    fn expect_word(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_word".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_identifier(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "word".to_string()), true));
            }
        }
        None
    }
    fn expect_string(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_string".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_string(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "string".to_string()), true));
            }
        }
        None
    }
    fn expect_number(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_number".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_number(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "number".to_string()), true));
            }
        }
        None
    }
    fn expect_tag(&mut self, arg: String) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_tag".to_string(), Some(arg.clone())) {
            return Some(memo);
        }
        if !self.tokens.at_end() {
            let current_token = self.tokens.peek_back(1).unwrap();
            return Some((Node {
                name: "error".to_string(),
                tokens: vec![Token { 
                    source: arg, index: 
                    current_token.index, line:
                    current_token.line 
                }],
                children: vec![],
            }, false));
        }
        None
    }
    fn expect_any(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_any".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.next() {
            return Some((node_from_token(next, "any".to_string()), false));
        }
        None
    }
    fn reset_parse(&mut self, item: &mut Node, pos: usize) {
        item.tokens.clear();
        item.children.clear();
        self.tokens.goto(pos);
    }
    fn memoize(&mut self, func: String, arg: Option<String>) -> Option<(Node, bool)> {
        let pos = self.tokens.index();
        let key = format!("{}({:?})", func, arg);
        if self.memos.has_memo(pos, key.clone()) {
            let memo = self.memos.get_memo(pos, key).unwrap();
            self.tokens.goto(memo.1);
            return memo.0.clone();
        } else {
            return None;
        }
    }
    fn make_memo(
        &mut self,
        pos: usize,
        func: String,
        result: Option<(Node, bool)>,
    ) -> Option<(Node, bool)> {
        let end_pos = self.tokens.index();
        let next_result = ParseResult(result, end_pos);
        self.memos.add_memo(pos, func, next_result).0
    }
    
    /*RULES*/

}

fn node_from_token(token: Token, name: String) -> Node {
    Node { 
        name,
        tokens: vec![token], 
        children: vec![] 
    }
}