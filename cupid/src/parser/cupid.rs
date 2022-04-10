#![allow(clippy::all)]
use crate::{
    is_identifier, is_number, is_string, is_uppercase, BiDirectionalIterator, Token, Tokenizer,
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
                return Some((
                    node_from_token(self.tokens.next().unwrap(), rule_name),
                    true,
                ));
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
            return Some((
                Node {
                    name: "error".to_string(),
                    tokens: vec![Token {
                        source: arg,
                        index: current_token.index,
                        line: current_token.line,
                    }],
                    children: vec![],
                },
                false,
            ));
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

    pub fn _file(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("file".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "file".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._expression(None), false);
            use_repeat!(&mut node, self._expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "file".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "file".to_string(), None);
        None
    }

    pub fn _expression(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expression".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "expression".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._expression_item(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "expression".to_string(), None);
        None
    }

    pub fn _expression_item(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("expression_item".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "expression_item".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._loop(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._declaration(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._symbol_declaration(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._function(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._assignment(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._operation(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._term(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "expression_item".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "expression_item".to_string(), None);
        None
    }

    pub fn _loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("loop".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "loop".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._for_loop(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._while_loop(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._infinite_loop(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "loop".to_string(), None);
        None
    }

    pub fn _for_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("for_loop".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "for_loop".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("for".to_string()), true);
            use_item!(&mut node, self._identifier(None), false);
            use_item!(&mut node, self.expect("in".to_string()), true);
            use_item!(&mut node, self._term(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "for_loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "for_loop".to_string(), None);
        None
    }

    pub fn _while_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("while_loop".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "while_loop".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("while".to_string()), true);
            use_item!(&mut node, self._expression_item(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "while_loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "while_loop".to_string(), None);
        None
    }

    pub fn _infinite_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("infinite_loop".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "infinite_loop".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("loop".to_string()), true);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "infinite_loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "infinite_loop".to_string(), None);
        None
    }

    pub fn _block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._if_block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._brace_block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._arrow_block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "block".to_string(), None);
        None
    }

    pub fn _if_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("if_block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "if_block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("if".to_string()), true);
            use_item!(&mut node, self._expression(None), false);
            use_item!(&mut node, self._block(None), false);
            use_repeat!(&mut node, self._else_if_block(None), false);
            use_optional!(&mut node, self._else_block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "if_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "if_block".to_string(), None);
        None
    }

    pub fn _else_if_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("else_if_block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "else_if_block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("else".to_string()), true);
            use_item!(&mut node, self.expect("if".to_string()), true);
            use_item!(&mut node, self._expression(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "else_if_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "else_if_block".to_string(), None);
        None
    }

    pub fn _else_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("else_block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "else_block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("else".to_string()), true);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "else_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "else_block".to_string(), None);
        None
    }

    pub fn _brace_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("brace_block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "brace_block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("{".to_string()), true);
            use_repeat!(&mut node, self._expression(None), false);
            use_item!(&mut node, self._closing_brace(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "brace_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "brace_block".to_string(), None);
        None
    }

    pub fn _arrow_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("arrow_block".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "arrow_block".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._arrow(None), true);
            use_item!(&mut node, self._require_expression(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "arrow_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "arrow_block".to_string(), None);
        None
    }

    pub fn _declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("declaration".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "declaration".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._symbol_declaration(None), false);
            use_item!(&mut node, self.expect("=".to_string()), true);
            use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(">".to_string()));
            use_item!(&mut node, self._expression_item(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "declaration".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "declaration".to_string(), None);
        None
    }

    pub fn _symbol_declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("symbol_declaration".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "symbol_declaration".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._keyword_variable(None), false);
            use_optional!(&mut node, self.expect("mut".to_string()), false);
            use_item!(&mut node, self._identifier(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "symbol_declaration".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "symbol_declaration".to_string(), None);
        None
    }

    pub fn _assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("assignment".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "assignment".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._identifier(None), false);
            use_item!(&mut node, self.expect("=".to_string()), false);
            use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(">".to_string()));
            use_item!(&mut node, self._expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "assignment".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "assignment".to_string(), None);
        None
    }

    pub fn _term(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("term".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "term".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._group(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._function_call(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._boolean(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._none(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._string(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._decimal(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._number(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._identifier(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._comment(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "term".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "term".to_string(), None);
        None
    }

    pub fn _group(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("group".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "group".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("(".to_string()), true);
            use_item!(&mut node, self._expression_item(None), false);
            use_item!(&mut node, self._closing_paren(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "group".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "group".to_string(), None);
        None
    }

    pub fn _function(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("function".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "function".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._parameters(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "function".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "function".to_string(), None);
        None
    }

    pub fn _parameters(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("parameters".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "parameters".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            loop {
                use_item!(&mut node, self._identifier(None), false);
                use_item!(&mut node, self.expect(",".to_string()), true);
            }
            let result = Some((node, false));
            return self.make_memo(start_pos, "parameters".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "parameters".to_string(), None);
        None
    }

    pub fn _function_call(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("function_call".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "function_call".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._identifier(None), false);
            use_item!(&mut node, self.expect("(".to_string()), true);
            use_item!(&mut node, self._arguments(None), false);
            use_item!(&mut node, self._closing_paren(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "function_call".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "function_call".to_string(), None);
        None
    }

    pub fn _arguments(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("arguments".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "arguments".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            loop {
                use_item!(&mut node, self._term(None), false);
                use_item!(&mut node, self.expect(",".to_string()), true);
            }
            let result = Some((node, false));
            return self.make_memo(start_pos, "arguments".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "arguments".to_string(), None);
        None
    }

    pub fn _operation(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("operation".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "operation".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._binary_op(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "operation".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._unary_op(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "operation".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "operation".to_string(), None);
        None
    }

    pub fn _binary_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("binary_op".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "binary_op".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._term(None), false);
            use_item!(&mut node, self._binary_operator(None), false);
            use_item!(&mut node, self._require_expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "binary_op".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "binary_op".to_string(), None);
        None
    }

    pub fn _binary_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("binary_operator".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "binary_operator".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("+".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "binary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("-".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "binary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("*".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "binary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("/".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "binary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._keyword_operator(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "binary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "binary_operator".to_string(), None);
        None
    }

    pub fn _unary_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("unary_op".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "unary_op".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._unary_operator(None), false);
            use_item!(&mut node, self._require_expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "unary_op".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "unary_op".to_string(), None);
        None
    }

    pub fn _unary_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("unary_operator".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "unary_operator".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("+".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "unary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("-".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "unary_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "unary_operator".to_string(), None);
        None
    }

    pub fn _boolean(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("boolean".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "boolean".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("true".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "boolean".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("false".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "boolean".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "boolean".to_string(), None);
        None
    }

    pub fn _none(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("none".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "none".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("none".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "none".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "none".to_string(), None);
        None
    }

    pub fn _identifier(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("identifier".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "identifier".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_negative_lookahead!(self, self.tokens.index(), &mut self._keyword(None));
            use_item!(&mut node, self.expect_word(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "identifier".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "identifier".to_string(), None);
        None
    }

    pub fn _string(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("string".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "string".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect_string(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "string".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "string".to_string(), None);
        None
    }

    pub fn _decimal(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("decimal".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "decimal".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect_number(None), false);
            use_item!(&mut node, self.expect(".".to_string()), true);
            use_item!(&mut node, self.expect_number(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "decimal".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "decimal".to_string(), None);
        None
    }

    pub fn _number(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("number".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "number".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect_number(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "number".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "number".to_string(), None);
        None
    }

    pub fn _require_expression(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("require_expression".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "require_expression".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._expression_item(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "require_expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(
                &mut node,
                self.expect_tag("<e missing_expression>".to_string()),
                false
            );

            let result = Some((node, true));
            return self.make_memo(start_pos, "require_expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "require_expression".to_string(), None);
        None
    }

    pub fn _closing_paren(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("closing_paren".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "closing_paren".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect(")".to_string()), true);

            let result = Some((node, true));
            return self.make_memo(start_pos, "closing_paren".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(
                &mut node,
                self.expect_tag("<e missing_closing_parenthesis>".to_string()),
                false
            );

            let result = Some((node, true));
            return self.make_memo(start_pos, "closing_paren".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "closing_paren".to_string(), None);
        None
    }

    pub fn _closing_brace(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("closing_brace".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "closing_brace".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("}".to_string()), true);

            let result = Some((node, true));
            return self.make_memo(start_pos, "closing_brace".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(
                &mut node,
                self.expect_tag("<e missing_closing_brace>".to_string()),
                false
            );

            let result = Some((node, true));
            return self.make_memo(start_pos, "closing_brace".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "closing_brace".to_string(), None);
        None
    }

    pub fn _keyword(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("keyword".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "keyword".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._keyword_variable(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._keyword_operator(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._keyword_control(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._type(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._boolean(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._none(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("for".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("while".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("else".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("if".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("loop".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "keyword".to_string(), None);
        None
    }

    pub fn _keyword_variable(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("keyword_variable".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "keyword_variable".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("let".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_variable".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("const".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_variable".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "keyword_variable".to_string(), None);
        None
    }

    pub fn _keyword_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("keyword_operator".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "keyword_operator".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("in".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("is".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("and".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("not".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("or".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect(">".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect(">".to_string()), false);
            use_item!(&mut node, self.expect("=".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("<".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("<".to_string()), false);
            use_item!(&mut node, self.expect("=".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "keyword_operator".to_string(), None);
        None
    }

    pub fn _keyword_control(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("keyword_control".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "keyword_control".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("break".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_control".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("return".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_control".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "keyword_control".to_string(), None);
        None
    }

    pub fn _arrow(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("arrow".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "arrow".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("=".to_string()), true);
            use_item!(&mut node, self.expect(">".to_string()), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "arrow".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "arrow".to_string(), None);
        None
    }

    pub fn _type(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("type".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "type".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._none(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("bool".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("int".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("dec".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("string".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("fun".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("maybe".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("list".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("dict".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self.expect("tuple".to_string()), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "type".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "type".to_string(), None);
        None
    }

    pub fn _comment_delimiter(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("comment_delimiter".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "comment_delimiter".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect("*".to_string()), false);
            use_item!(&mut node, self.expect("*".to_string()), false);
            use_item!(&mut node, self.expect("*".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "comment_delimiter".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "comment_delimiter".to_string(), None);
        None
    }

    pub fn _comment_content(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("comment_content".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "comment_content".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self.expect_any(None), false);
            use_negative_lookahead!(
                self,
                self.tokens.index(),
                &mut self._comment_delimiter(None)
            );

            let result = Some((node, true));
            return self.make_memo(start_pos, "comment_content".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "comment_content".to_string(), None);
        None
    }

    pub fn _comment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("comment".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "comment".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._comment_delimiter(None), true);
            use_repeat!(&mut node, self._comment_content(None), false);
            use_item!(&mut node, self.expect_any(None), false);
            use_item!(&mut node, self._comment_delimiter(None), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "comment".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "comment".to_string(), None);
        None
    }
}

fn node_from_token(token: Token, name: String) -> Node {
    Node {
        name,
        tokens: vec![token],
        children: vec![],
    }
}
