use crate::{is_identifier, is_number, is_string, is_uppercase, BiDirectionalIterator, Tokenizer};
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
    ($parser:expr, $item:ident, $method:expr) => {{
        let pos = $parser.tokens.index();
        if let Some((mut val, pass_through)) = $method {
            $parser.tokens.goto(pos);
            break;
        }
    }};
}

macro_rules! use_positive_lookahead {
    ($parser:expr, $item:ident, $method:expr) => {{
        let pos = $parser.tokens.index();
        if let Some((mut val, pass_through)) = $method {
        } else {
            $parser.tokens.goto(pos);
            break;
        }
    }};
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<String>,
}

#[derive(PartialEq, Eq)]
pub struct Parser {
    pub tokens: BiDirectionalIterator<String>,
    pub index: usize,
    pub memos: Memos,
}

#[derive(PartialEq, Eq)]
pub struct Memos {
    memos: HashMap<usize, HashMap<String, ParseResult>>,
}

impl Memos {
    fn has_memo(&self, pos: usize) -> bool {
        return self.memos.contains_key(&pos);
    }
    fn get_memo(&mut self, pos: usize, func: String) -> Option<&ParseResult> {
        return self.memos.entry(pos).or_insert(HashMap::new()).get(&func);
    }
    fn add_memo(&mut self, pos: usize, func: String, result: ParseResult) -> ParseResult {
        return self
            .memos
            .entry(pos)
            .or_insert(HashMap::new())
            .entry(func)
            .insert_entry(result)
            .get()
            .clone();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParseResult(Option<(Node, bool)>, usize);

impl Parser {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Tokenizer::new(source.as_str());
        tokenizer.scan();
        println!("{:?}", tokenizer.tokens);
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
        if let Some(token) = self.tokens.expect(rule_name.clone()) {
            return Some((node_from_token(token, rule_name), true));
        }
        None
    }
    fn expect_one(&mut self, rule_names: Vec<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_one".to_string(), Some(format!("{:?}", rule_names))) {
            return Some(memo);
        }
        for rule_name in rule_names {
            if let Some(next) = self.expect(rule_name) {
                return Some(next);
            }
        }
        return None;
    }
    fn expect_constant(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_constant".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_uppercase(next.clone()) {
                if let Some(token) = self.tokens.next() {
                    return Some((node_from_token(token, "constant".to_string()), true));
                }
            }
        }
        return None;
    }
    fn expect_word(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_word".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_identifier(next.clone()) {
                if let Some(token) = self.tokens.next() {
                    return Some((node_from_token(token, "word".to_string()), true));
                }
            }
        }
        return None;
    }
    fn expect_string(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_string".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_string(next.clone()) {
                if let Some(token) = self.tokens.next() {
                    return Some((node_from_token(token, "string".to_string()), true));
                }
            }
        }
        return None;
    }
    fn expect_number(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expect_number".to_string(), None) {
            return Some(memo);
        }
        if let Some(next) = self.tokens.peek(0) {
            if is_number(next.clone()) {
                if let Some(token) = self.tokens.next() {
                    return Some((node_from_token(token, "number".to_string()), true));
                }
            }
        }
        return None;
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
            return None
        }
    }
    fn make_memo(&mut self, pos: usize, func: String, result: Option<(Node, bool)>) -> Option<(Node, bool)> {
        let end_pos = self.tokens.index();
        let next_result = ParseResult(result, end_pos);
        return self.memos.add_memo(pos, func, next_result).0;
    }
    
    /*RULES*/

}

fn node_from_token(token: String, name: String) -> Node {
    Node { 
        name,
        tokens: vec![token], 
        children: vec![] 
    }
}