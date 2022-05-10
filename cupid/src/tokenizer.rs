use std::fmt::{Display, Formatter, Result};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};

pub struct Tokenizer<'src> {
	pub source: Cow<'src, str>,
	pub index: usize,
	pub chars: Vec<char>,
	pub tokens: Vec<Token<'src>>,
	pub line: usize,
	pub line_index: usize,
	pub bracket_stack: Vec<char>,
}

const OPEN: [char; 3] = ['[', '(', '{'];
const CLOSE: [char; 3] = [']', ')', '}'];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Token<'src> {
	pub source: Cow<'src, str>,
	pub line: usize,
	pub index: usize
}

impl<'src> Tokenizer<'src> {
	pub fn new(string: Cow<'src, str>) -> Self {
		Self {
			chars: string.chars().collect(),
			source: string,
			index: 0,
			tokens: vec![],
			line: 1,
			line_index: 0,
			bracket_stack: vec![],
		}
	}
	pub fn add_token(&mut self, source: String) {
		let index = if self.line_index > source.len() { 
			self.line_index - source.len()
		} else { 0 };
		self.tokens.push(Token { source: source.into(), index, line: self.line });
	}
	pub fn current(&self) -> &char { self.chars.get(self.index).unwrap_or(&'\0') }
	pub fn is_done(&self) -> bool { self.index >= self.chars.len() }
	pub fn peek(&self, amount: usize) -> Option<&char> { self.chars.get(self.index + amount) }
	pub fn advance(&mut self) -> &char {
		self.index += 1;
		self.line_index += 1;
		self.current()
	}
	pub fn scan(&mut self) {
		
		while !self.is_done() {
			let c = *self.current();
			self.handle_bracket(c);
			
			match c {
				'#' => self.line_comment(),
				'<' => self.tag(c),
				'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),
				'0'..='9' => self.identifier(c),
				'\'' | '"' => self.string(c), //"
				' ' | '\t' => (), // ignore whitespace
				'\n' => {
					self.line += 1;
					self.line_index = 0;
				}
				_ => self.add_token(c.to_string())
			}
			self.advance();
		}
		self.add_token("EOF".to_string());
	}
	fn line_comment(&mut self) {
		while let Some(c) = self.peek(1) {
			if *c == '\n' || self.is_done() { break; }
			self.advance();
		}
	}
	fn identifier(&mut self, start_char: char) {
		let mut source = start_char.to_string();
		while let Some(c) = self.peek(1) {
			if c.is_alphanumeric() || *c == '_' {
				source += &mut *self.advance().encode_utf8(&mut [0; 4]);
			} else { break; }
		}
		self.add_token(source);
	}
	fn string(&mut self, start_char: char) {
		let mut source = start_char.to_string();
		while let Some(c) = self.peek(1) {
			if *c == start_char { break; }
			source.push(*self.advance()); 
		}
		source.push(*self.advance()); 
		self.add_token(source);
	}
	fn tag(&mut self, start_char: char) {
		let mut source = start_char.to_string();
		
		// tags can be either <e 'message'> or <w 'message'>
		// error or warning
		if let Some(c) = self.peek(1) {
			if *c == 'e' || *c == 'w' {
				source.push(*self.advance());
			} else {
				self.add_token(source);
				return 
			}
		}
		
		while let Some(c) = self.peek(1) {
			if *c == '>' || self.is_done() { break; }
			source.push(*self.advance()); 
		}
		source.push(*self.advance()); 
		self.add_token(source);
	}
	pub fn handle_bracket(&mut self, c: char) {		
		if OPEN.contains(&c) {
			self.bracket_stack.push(c);
		}
		if CLOSE.contains(&c) {
			let match_index = OPEN.iter().position(|i| *i == c);
			if let Some(i) = match_index {
				let last_bracket = self.bracket_stack.last().unwrap_or(&'\0');
				let close_match_index = CLOSE.iter().position(|i| *i == *last_bracket);
				if let Some(close_index) = close_match_index {
					if *last_bracket != OPEN[i] {
						self.add_token(format!(
							"<e 'Expected closing bracket `{}`, not `{}`'>", 
							CLOSE[close_index],
							c
						))
					} else {
						self.bracket_stack.pop();
					}
				}
			}
		}
	}
}

impl<'src> Display for Token<'src> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		writeln!(f, "Token `{}` ({}:{})", self.source, self.line, self.index)
	}
}

pub struct TokenList<'src>(Vec<Token<'src>>);

impl<'src> Display for TokenList<'src> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		_ = writeln!(f, "Tokens:");
    	self.0.iter().for_each(|t| { _ = write!(f, "{}", t); });
		_ = writeln!(f);
		Ok(())
	}
}