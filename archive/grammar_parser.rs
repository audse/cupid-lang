use crate::Tokenizer;

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
	pub name: String,
	pub alts: Vec<Vec<Alt>>,
}

pub struct GrammarParser {
	pub tokens: Vec<String>,
	pub index: usize
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alt {
	pub kind: String,
	pub source: String,
	pub suffix_modifier: Option<String>,
	pub prefix_modifier: Option<String>
}

impl GrammarParser {
	pub fn new(source: String) -> Self {
		let mut tokenizer = Tokenizer::new(source.as_str());
		tokenizer.scan();
		Self {
			index: 0,
			tokens: tokenizer.tokens,
		}
	}
	fn mark(&self) -> usize { return self.index; }
	fn reset(&mut self, pos: usize) { self.index = pos; }
	fn current(&self) -> String { 
		self.tokens[self.index].clone() 
	}
	fn expect(&mut self, rule_name: &str) -> Option<String> {
		let next_rule = self.current();
		if rule_name == next_rule.as_str() {
			self.index = self.index + 1;
			return Some(next_rule);
		}
		return None;
	}
	fn expect_one(&mut self, rule_names: Vec<&str>) -> Option<String> {
		let next_rule = self.current();
		for rule in rule_names {
			if rule == next_rule {
				self.index = self.index + 1;
				return Some(next_rule);
			}
		}
		return None;
	}
	fn peek_expect(&self, rule: &str) -> Option<String> {
		let next = self.current();
		if next == rule {
			return Some(next.clone());
		}
		return None;
	}
	fn expect_constant(&mut self) -> Option<String> {
		let next = self.current();
		let is_const = next.chars().all(|c| match c {
			'A'..='Z' => true,
			_ => false
		});
		if is_const {
			self.index += 1;
			return Some(next);
		}
		return None;
	}
	fn expect_name(&mut self) -> Option<String> {
		let next = self.current();
		let first_char = next.chars().next().unwrap_or('\0');
		let is_name = match first_char {
			'a'..='z' | 'A'..='Z' | '_' => true,
			_ => false
		};
		if is_name { 
			self.index = self.index + 1;
			return Some(next.to_string()); 
		}
		return None;
	}
	fn expect_string(&mut self) -> Option<String> {
		let rule = self.current();
		let first_char = rule.chars().next().unwrap_or('\0');
		let is_name = match first_char {
			'\'' | '"' => true, //"
			_ => false
		};
		if is_name { 
			self.index = self.index + 1;
			return Some(rule.to_string()); 
		}
		return None;
	}
	
	pub fn grammar(&mut self) -> Vec<Rule> {
		let pos = self.mark();
		let mut rules = vec![];
		loop {
			let rule = self.rule();
			match rule {
				Some(r) => rules.push(r),
				_ => break
			}
			if let Some(_end) = self.expect("EOF") {
				return rules
			}
		}
		self.reset(pos);
		return vec![];
	}
	
	fn rule(&mut self) -> Option<Rule> {
		let pos = self.mark();
		
		if let Some(_dash) = self.expect("-") {
			if let Some(name) = self.expect_name() {
				if let Some(_colon) = self.expect(":") {
					let mut alts = vec![self.alternative()];
					let mut alt_pos = self.mark();
					
					while let Some(_option) = self.expect("|") {
						let next_alts = self.alternative();
						alts.push(next_alts);
						alt_pos = self.mark();
					}
					
					self.reset(alt_pos);
					if let Some(_end) = self.peek_expect("-") {
						return Some(Rule { name, alts });
					}
					if let Some(_end) = self.peek_expect("EOF") {
						return Some(Rule { name, alts });
					}
				}
			}
		}
		
		self.reset(pos);
		return None;
	}
	
	fn alternative(&mut self) -> Vec<Alt> {
		let mut items = vec![];
		loop {
			if let Some(item) = self.item() {
				items.push(item);
			} else {
				break;
			}
		}
		return items;
	}
	
	fn item(&mut self) -> Option<Alt> {
		let prefix_modifier = self.prefix_modifier();
		if let Some(con) = self.expect_constant() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "constant".to_string(), source: con, prefix_modifier, suffix_modifier });
		}
		if let Some(name) = self.expect_name() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "name".to_string(), source: name, prefix_modifier, suffix_modifier });
		}
		if let Some(string) = self.expect_string() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "string".to_string(), source: string, prefix_modifier, suffix_modifier });
		}
		return None;
	}
	
	fn prefix_modifier(&mut self) -> Option<String> {
		return self.expect_one(vec!["~", "&", "!"]);
	}
	
	fn suffix_modifier(&mut self) -> Option<String> {
		return self.expect_one(vec!["?", "*", "+"]);
	}
}