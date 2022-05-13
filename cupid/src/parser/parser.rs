use crate::*;

pub trait Parser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token>;
	
	fn build(source: String) -> BiDirectionalIterator<Token> {
		let mut tokenizer = Tokenizer::new(source.into());
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
			let current_token = self.tokens().peek_back(1).unwrap();
			return Some((
				ParseNode {
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

macro_rules! alt {
	(($parser:tt, $pass_through:tt, $node:ident, $pos:ident) { $($item:stmt;)* }) => {{
		loop {
			$($item)*
			return Some(($node, $pass_through));
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

macro_rules! optional {
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

macro_rules! repeat {
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

pub(crate) use alt;
pub(crate) use group;
pub(crate) use once;
pub(crate) use optional;
pub(crate) use repeat;
pub(crate) use use_negative_lookahead;
pub(crate) use use_positive_lookahead;