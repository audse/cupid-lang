pub struct Tokenizer {
	pub source: String,
	pub index: usize,
	pub chars: Vec<char>,
	pub tokens: Vec<String>,
}

impl Tokenizer {
	pub fn new(string: &str) -> Self {
		Self {
			source: string.to_string(),
			index: 0,
			chars: string.clone().chars().collect(),
			tokens: vec![]
		}
	}
	pub fn current(&self) -> &char { self.chars.get(self.index).unwrap_or(&'\0') }
	pub fn is_done(&self) -> bool { self.index >= self.chars.len() }
	pub fn peek(&self, amount: usize) -> Option<&char> { self.chars.get(self.index + amount) }
	pub fn advance(&mut self) -> &char {
		self.index += 1;
		return self.current();
	}
	pub fn scan(&mut self) -> () {
		while !self.is_done() {
			let c = *self.current();
			match c {
				'#' => self.line_comment(),
				'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),
				'0'..='9' => self.identifier(c),
				'\'' | '"' => self.string(c), //"
				' ' | '\t' | '\r' | '\n' => (), // ignore whitespace
				_ => self.tokens.push(c.to_string())
			}
			self.advance();
		}
		self.tokens.push("EOF".to_string());
	}
	fn identifier(&mut self, start_char: char) {
		let mut source = start_char.to_string();
		loop {
			if let Some(c) = self.peek(1) {
				if c.is_alphanumeric() || *c == '_' {
					source.push(*self.advance());
				} else { break; }
			} else { break; }
		}
		self.tokens.push(source);
	}
	fn string(&mut self, start_char: char) {
		let mut source = start_char.to_string();
		loop {
			if let Some(c) = self.peek(1) {
				if *c == start_char { break; }
				source.push(*self.advance()); 
			} else { break; }
		}
		source.push(*self.advance()); 
		self.tokens.push(source);
	}
	fn line_comment(&mut self) {
		loop {
			if let Some(p) = self.peek(1) {
				if *p == '\n' || self.is_done() { break; }
				self.advance();
			} else { break; }
		}
	}
}