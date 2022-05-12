#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;
use serde::{Deserialize, Serialize};

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
		let index = $index;
		if let Some((_val, _pass_through)) = $method {
			$parser.tokens.goto(index);
			break;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Node {
	pub name: Cow<'static, str>,
	pub children: Vec<Node>,
	pub tokens: Vec<Token>,
}

impl Node {
	pub fn map<R>(&self, function: &dyn Fn(&Self) -> R) -> Vec<R> {
		self.children.iter().map(function).collect()
	}
	pub fn map_mut<R>(&mut self, function: &dyn Fn(&mut Self) -> R) -> Vec<R> {
		self.children.iter_mut().map(function).collect()
	}
	pub fn filter_map_mut<R>(&mut self, function: &dyn Fn(&mut Self) -> Option<R>) -> Vec<R> {
		self.children.iter_mut().filter_map(function).collect()
	}
	pub fn has(&self, name: &str) -> bool {
		self.children.iter().find(|c| c.name == name).is_some()
	}
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Self> {
		self.children.iter_mut().find(|c| c.name == name)
	}
}

#[derive(PartialEq, Eq)]
pub struct Parser {
	pub tokens: BiDirectionalIterator<Token>,
	pub index: usize,
}

impl Parser {
	pub fn new(source: String) -> Self {
		let mut tokenizer = Tokenizer::new(source.into());
		tokenizer.scan();
		Self {
			index: 0,
			tokens: BiDirectionalIterator::new(tokenizer.tokens),
		}
	}

	#[inline]
	fn expect(&mut self, rule_name: &'static str) -> Option<(Node, bool)> {
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

	#[inline]
	fn expect_one(&mut self, rule_names: Vec<&'static str>) -> Option<(Node, bool)> {
		for rule_name in rule_names {
			if let Some(next) = self.expect(rule_name) {
				return Some(next);
			}
		}
		None
	}

	#[inline]
	fn expect_constant(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.peek(0) {
			if is_uppercase(&next.source) {
				let token = self.tokens.next().unwrap();
				return Some((node_from_token(token, "constant"), true));
			}
		}
		None
	}

	#[inline]
	fn expect_word(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.peek(0) {
			if is_identifier(&next.source) {
				let token = self.tokens.next().unwrap();
				return Some((node_from_token(token, "word"), true));
			}
		}
		None
	}

	#[inline]
	fn expect_string(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.peek(0) {
			if is_string(&next.source) {
				let token = self.tokens.next().unwrap();
				return Some((node_from_token(token, "string"), true));
			}
		}
		None
	}

	#[inline]
	fn expect_letter(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.peek(0) {
			if next.source.len() == 1 {
				let token = self.tokens.next().unwrap();
				return Some((node_from_token(token, "letter"), true));
			}
		}
		None
	}

	#[inline]
	fn expect_number(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.peek(0) {
			if is_number(&next.source) {
				let token = self.tokens.next().unwrap();
				return Some((node_from_token(token, "number"), true));
			}
		}
		None
	}

	#[inline]
	fn expect_tag(&mut self, arg: &'static str) -> Option<(Node, bool)> {
		if !self.tokens.at_end() {
			let current_token = self.tokens.peek_back(1).unwrap();
			return Some((
				Node {
					name: Cow::Borrowed("error"),
					tokens: vec![
						Token {
							source: Cow::Borrowed(arg),
							index: current_token.index + 1,
							line: current_token.line,
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
	fn expect_any(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
		if let Some(next) = self.tokens.next() {
			return Some((node_from_token(next, "any"), false));
		}
		None
	}

	#[inline]
	fn reset_parse(&mut self, item: &mut Node, pos: usize) {
		item.tokens.clear();
		item.children.clear();
		self.tokens.goto(pos);
	}
    
    
		pub fn _packages(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "packages".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_repeat!(&mut node, self._package(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _package(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "package".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("package"), false);
use_optional!(&mut node, self._name_space(None), false);
use_item!(&mut node, self._items(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _name_space(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "name_space".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._item(None), false);
use_item!(&mut node, self.expect(":"), false);
use_item!(&mut node, self.expect(":"), false);
use_optional!(&mut node, self._name_space(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _items(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "items".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._item_group(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._item(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _item_group(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "item_group".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._item(None), false);
use_item!(&mut node, self.expect(","), false);
}use_item!(&mut node, self.expect("]"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _item(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "item".into(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_word(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		

}

fn node_from_token(token: Token, name: &'static str) -> Node {
	Node {
		name: Cow::Borrowed(name),
		tokens: vec![token],
		children: vec![],
	}
}