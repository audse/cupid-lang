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

macro_rules! reset_parse {
    ($parser:expr, $item:ident, $pos:expr) => {{
        $item.tokens.clear();
        $item.children.clear();
        $parser.tokens.goto($pos);
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
    fn has_memo(&self, pos: usize, func: String) -> bool {
        return self.memos.contains_key(&pos) && self.memos.get(&pos).unwrap().contains_key(&func);
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

// pub struct ParseFn(String, Box<dyn Fn(Option<String>) -> Option<(Node, bool)>>);
// impl PartialEq for ParseFn {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }
//
// impl Eq for ParseFn {}
//
// trait DynHash {
//     fn dyn_hash(&self, state: &mut dyn Hasher);
// }
//
// impl<H: Hash + ?Sized> DynHash for H {
//     fn dyn_hash(&self, mut state: &mut dyn Hasher) {
//         self.hash(&mut state);
//     }
// }
//
// impl Hash for ParseFn {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.dyn_hash(state);
//     }
// }

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
    // fn get_memoize(&mut self, func: ParseFn) -> Option<(Node, bool)> {
    //     return self.memoize(func).0;
    // }
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
        return self.memos.add_memo(pos, func, next_result).0;
    }

    // fn get_memoize(&mut self, func: ParseFn) -> Option<(Node, bool)> {
    //     return self.memoize(func).0;
    // }
    // fn memoize(&mut self, func: ParseFn) -> ParseResult {
    //     let pos = self.tokens.index();
    //
    //     if self.memos.has_memo(pos) {
    //         let memo = self.memos.get_memo(pos, &func).unwrap();
    //         self.tokens.goto(memo.1);
    //         return memo.clone();
    //     } else {
    //         return self.make_memo(pos, func);
    //     }
    // }
    // fn make_memo(&mut self, pos: usize, func: ParseFn) -> ParseResult {
    //     let result_val = match &func {
    //         ParseFn(f, a) => f(*a),
    //     };
    //     let end_pos = self.tokens.index();
    //     let result = ParseResult(result_val, end_pos);
    //     return self.memos.add_memo(pos, func, result);
    // }

    pub fn _expression(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._loop(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._declaration(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._assignment(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._term(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "expression".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "expression".to_string(), None);
        return None;
    }

    pub fn _loop(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _for_loop(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _while_loop(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._expression(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "while_loop".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "while_loop".to_string(), None);
        return None;
    }

    pub fn _infinite_loop(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _block(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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

            let result = Some((node, true));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._brace_block(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        loop {
            use_item!(&mut node, self._arrow_block(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "block".to_string(), None);
        return None;
    }

    pub fn _if_block(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_optional!(&mut node, self._else_block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "if_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "if_block".to_string(), None);
        return None;
    }

    pub fn _else_block(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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

            let result = Some((node, true));
            return self.make_memo(start_pos, "else_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "else_block".to_string(), None);
        return None;
    }

    pub fn _brace_block(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self.expect("}".to_string()), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "brace_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "brace_block".to_string(), None);
        return None;
    }

    pub fn _arrow_block(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "arrow_block".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "arrow_block".to_string(), None);
        return None;
    }

    pub fn _declaration(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._keyword_variable(None), false);
            use_optional!(&mut node, self.expect("mut".to_string()), false);
            use_item!(&mut node, self._assignment(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "declaration".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "declaration".to_string(), None);
        return None;
    }

    pub fn _assignment(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self.expect("=".to_string()), true);
            use_item!(&mut node, self._expression(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "assignment".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "assignment".to_string(), None);
        return None;
    }

    pub fn _term(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._anonymous_function(None), false);

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
        self.make_memo(start_pos, "term".to_string(), None);
        return None;
    }

    pub fn _group(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._expression(None), false);
            use_item!(&mut node, self.expect(")".to_string()), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "group".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "group".to_string(), None);
        return None;
    }

    pub fn _anonymous_function(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("anonymous_function".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "anonymous_function".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_optional!(&mut node, self._parameters(None), false);
            use_item!(&mut node, self._arrow(None), false);
            use_item!(&mut node, self._block(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "anonymous_function".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "anonymous_function".to_string(), None);
        return None;
    }

    pub fn _parameters(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._identifier(None), false);
            use_item!(&mut node, self.expect(",".to_string()), false);
            loop {
                use_item!(&mut node, self._identifier(None), false);
                use_item!(&mut node, self.expect(",".to_string()), false);
            }
            let result = Some((node, false));
            return self.make_memo(start_pos, "parameters".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "parameters".to_string(), None);
        return None;
    }

    pub fn _function_call(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_optional!(&mut node, self._args(None), false);
            use_item!(&mut node, self.expect(")".to_string()), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "function_call".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "function_call".to_string(), None);
        return None;
    }

    pub fn _args(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(memo) = self.memoize("args".to_string(), None) {
            return Some(memo);
        }
        let start_pos = self.tokens.index();
        let pos = start_pos;
        let mut node = Node {
            name: "args".to_string(),
            tokens: vec![],
            children: vec![],
        };
        loop {
            use_item!(&mut node, self._term(None), false);
            use_item!(&mut node, self.expect(",".to_string()), true);
            loop {
                use_item!(&mut node, self._term(None), false);
                use_item!(&mut node, self.expect(",".to_string()), true);
            }
            let result = Some((node, false));
            return self.make_memo(start_pos, "args".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "args".to_string(), None);
        return None;
    }

    pub fn _boolean(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _none(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self.expect("none".to_string()), true);

            let result = Some((node, false));
            return self.make_memo(start_pos, "none".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "none".to_string(), None);
        return None;
    }

    pub fn _identifier(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self.expect_word(None), false);

            let result = Some((node, false));
            return self.make_memo(start_pos, "identifier".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "identifier".to_string(), None);
        return None;
    }

    pub fn _string(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _decimal(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _number(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _keyword_variable(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }

    pub fn _keyword_operator(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self._arrow(None), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "keyword_operator".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "keyword_operator".to_string(), None);
        return None;
    }

    pub fn _arrow(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
            use_item!(&mut node, self.expect("=>".to_string()), false);

            let result = Some((node, true));
            return self.make_memo(start_pos, "arrow".to_string(), result);
        }
        self.reset_parse(&mut node, pos);
        self.make_memo(start_pos, "arrow".to_string(), None);
        return None;
    }

    pub fn _type(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
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
        return None;
    }
}

fn node_from_token(token: String, name: String) -> Node {
    Node {
        name,
        tokens: vec![token],
        children: vec![],
    }
}
