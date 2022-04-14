use std::fmt::{Display, Formatter, Result};

pub struct Tokenizer {
	pub source: String,
	pub index: usize,
	pub chars: Vec<char>,
	pub tokens: Vec<Token>,
	pub line: usize,
	pub line_index: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
	pub source: String,
	pub line: usize,
	pub index: usize
}

impl Tokenizer {
	pub fn new(string: &str) -> Self {
		Self {
			source: string.to_string(),
			index: 0,
			chars: string.chars().collect(),
			tokens: vec![],
			line: 1,
			line_index: 0,
		}
	}
	pub fn add_token(&mut self, source: String) {
		let index = if self.line_index > source.len() { 
			self.line_index - source.len()
		} else { 0 };
		self.tokens.push(Token { source, index, line: self.line });
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
		// println!("{}", TokenList(self.tokens.clone()));
		self.add_token(String::from("EOF"));
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
				source.push(*self.advance());
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
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> Result {
		writeln!(f, "Token `{}` ({}:{})", self.source, self.line, self.index)
	}
}

pub struct TokenList(Vec<Token>);

impl Display for TokenList {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		_ = writeln!(f, "Tokens:");
    	self.0.iter().for_each(|t| { _ = write!(f, "{}", t); });
		_ = writeln!(f);
		Ok(())
	}
}