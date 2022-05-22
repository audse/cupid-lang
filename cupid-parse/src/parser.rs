#![allow(unused_macros)]
use crate::*;

pub trait Parser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token>;
	fn file(&self) -> usize;
	
	fn build(source: String, file: usize) -> BiDirectionalIterator<Token> {
		let mut tokenizer = Tokenizer::new(source.into(), file);
		tokenizer.scan();
		BiDirectionalIterator::new(tokenizer.tokens)
	}
	
	#[inline]
	fn expect(&mut self, rule_name: &'static str) -> Option<(ParseNode, bool)> {
		if let Some(token) = self.tokens().peek(0) {
			if token.source == rule_name {
				let node = (self.tokens().next().unwrap(), rule_name).into();
				return Some((node, true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_one(&mut self, rule_names: Vec<&'static str>) -> Option<(ParseNode, bool)> {
		for rule_name in rule_names {
			if let Some(next) = self.expect(rule_name) {
				return Some(next);
			}
		}
		None
	}
	
	#[inline]
	fn expect_constant(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if is_uppercase(&next.source) {
				let token = self.tokens().next().unwrap();
				return Some(((token, "constant").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_word(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if is_identifier(&next.source) {
				let token = self.tokens().next().unwrap();
				return Some(((token, "word").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_string(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if is_string(&next.source) {
				let token = self.tokens().next().unwrap();
				return Some(((token, "string").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_letter(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if next.source.len() == 1 {
				let token = self.tokens().next().unwrap();
				return Some(((token, "letter").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_number(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if is_number(&next.source) {
				let token = self.tokens().next().unwrap();
				return Some(((token, "number").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_tag(&mut self, arg: &'static str) -> Option<(ParseNode, bool)> {
		if !self.tokens().at_end() {
			let file = self.file();
			let current_token = self.tokens().peek_back(1).unwrap();
			return Some((
				ParseNode {
					name: Cow::Borrowed("error"),
					tokens: vec![
						Token {
							source: Cow::Borrowed(arg),
							index: current_token.index + 1,
							line: current_token.line,
							end_index: 0, // TODO
							end_line: 0, // TODO
							file
						},
						current_token.to_owned(),
					],
					children: vec![],
				},
				false,
			));
		}
		None
	}
	
	#[inline]
	fn expect_any_tag(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().peek(0) {
			if is_tag(&next.source) {
				let token = self.tokens().next().unwrap();
				return Some(((token, "tag").into(), true));
			}
		}
		None
	}
	
	#[inline]
	fn expect_any(&mut self) -> Option<(ParseNode, bool)> {
		if let Some(next) = self.tokens().next() {
			return Some(((next, "any").into(), false));
		}
		None
	}
	
	#[inline]
	fn start_parse(&mut self, name: &'static str) -> (ParseNode, usize) {
		let pos = self.tokens().index();
		let node = ParseNode {
			name: name.into(),
			tokens: vec![],
			children: vec![],
		};
		(node, pos)
	}
	
	#[inline]
	fn reset_parse(&mut self, item: &mut ParseNode, pos: usize) {
		item.tokens.clear();
		item.children.clear();
		self.tokens().goto(pos);
	}
}

impl From<(Token, &'static str)> for ParseNode {
	fn from(data: (Token, &'static str)) -> Self {
		ParseNode {
			name: Cow::Borrowed(data.1),
			tokens: vec![data.0],
			children: vec![],
		}
	}
}

macro_rules! add_child {
	($pass_through:tt, $is_concealed:expr, $item:expr, $val:expr) => {
		if $pass_through && !$is_concealed {
			// move returned node's children to current node
			$item.tokens.append(&mut $val.tokens);
			$item.children.append(&mut $val.children);
		} else if !$is_concealed {
			$item.children.push($val);
		}
	}
}

pub fn invert_binary_op(mut node: ParseNode) -> ParseNode {
	/* 
	Invert the precedence of a binary operation for left recursion
	ex. `person.name.first`
		 parses as:
			property[ "person", property["name", "first"] ]
		 and is transformed into
			property[ property["person", "name"], "first" ]
	*/
	let is_op = node.children.len() == 2 && node.children[1].children.len() == 2;
	if is_op {
		let mut right_node = node.children.pop().unwrap();
		let left_node = node.children.pop().unwrap();
		
		let r_right_node = right_node.children.pop().unwrap();
		let r_left_node = right_node.children.pop().unwrap();
		
		right_node.children.push(left_node);
		right_node.children.push(r_left_node);
		
		node.children.push(right_node);
		node.children.push(r_right_node);
	}
	node
}

macro_rules! alt {
	(($parser:tt, $pass_through:tt, $node:ident, $pos:ident) { $($item:stmt;)* }) => {{
		loop {
			$($item)*
			return Some(($node, $pass_through));
		}
		$parser.reset_parse(&mut $node, $pos);
	}}
}

macro_rules! alt_inverse {
	(($parser:tt, $pass_through:tt, $node:ident, $pos:ident) { $($item:stmt;)* }) => {{
		loop {
			$($item)*
			
			return Some((invert_binary_op($node), $pass_through));
		}
		$parser.reset_parse(&mut $node, $pos);
	}}
}

macro_rules! group {
	(($parser:tt, $pass_through:tt, $node:ident, $pos:ident) { $($item:stmt;)* }) => {{
		loop {
			$($item)*
		}
	}}
}

macro_rules! once {
	($item:expr, $method:expr, $is_concealed:expr) => {{
		if let Some((mut val, pass_through)) = $method {
			add_child!(pass_through, $is_concealed, $item, val);
		} else {
			break;
		}
	}};
}

macro_rules! optional {
	($item:expr, $method:expr, $is_concealed:expr) => {{
		if let Some((mut val, pass_through)) = $method {
			add_child!(pass_through, $is_concealed, $item, val);
		}
	}};
}

macro_rules! repeat {
	($item:expr, $method:expr, $is_concealed:expr) => {{
		while let Some((mut val, pass_through)) = $method {
			add_child!(pass_through, $is_concealed, $item, val);
		}
	}};
}

macro_rules! use_negative_lookahead {
	($parser:expr, $index:expr, $method:expr) => {{
		let index = $index;
		if let Some((_val, _pass_through)) = $method {
			$parser.tokens.goto(index);
			break;
		}
	}};
}

macro_rules! use_negative_lookbehind {
	($parser:expr, $index:expr, $method:expr) => {{
		let index = $index;
		if $parser.tokens.index() > 0 {
			$parser.tokens.goto(index - 1);
			if let Some((_val, _pass_through)) = $method {
				$parser.tokens.goto(index);
				break;
			} else {
				$parser.tokens.goto(index);
			}
		}
	}};
}


macro_rules! use_positive_lookahead {
	($parser:expr, $index:expr, $method:expr) => {{
		let index = $index;
		if let Some((_val, _pass_through)) = $method {
		} else {
			$parser.tokens.goto(index);
			break;
		}
	}};
}

macro_rules! use_positive_lookbehind {
	($parser:expr, $index:expr, $method:expr) => {{
		let index = $index;
		if $parser.tokens.index() > 0 {
			$parser.tokens.goto(index - 1);
			if let Some((_val, _pass_through)) = $method {
				$parser.tokens.goto(index);
			} else {
				$parser.tokens.goto(index);
				break;
			}
		}
	}};
}

pub(crate) use add_child;
pub(crate) use alt;
pub(crate) use alt_inverse;
pub(crate) use group;
pub(crate) use once;
pub(crate) use optional;
pub(crate) use repeat;
pub(crate) use use_negative_lookbehind;
pub(crate) use use_negative_lookahead;
pub(crate) use use_positive_lookahead;
pub(crate) use use_positive_lookbehind;