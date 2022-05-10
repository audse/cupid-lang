use crate::*;

pub struct GrammarParser<'src> {
	pub tokens: BiDirectionalIterator<Token<'src>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule<'src> {
	pub name: Cow<'src, str>,
	pub alts: Vec<Vec<AltGroup<'src>>>,
	pub pass_through: bool, // to be included in the rule tree, or "passed through" to an encapsulating rule
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alt<'src> {
	pub kind: Cow<'src, str>,
	pub source: Token<'src>,
	pub prefix_modifier: Option<Cow<'src, str>>,
	pub suffix_modifier: Option<Cow<'src, str>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AltGroup<'src> {
	pub alts: Vec<Alt<'src>>,
	pub prefix_modifier: Option<Cow<'static, str>>,
	pub suffix_modifier: Option<Cow<'static, str>>,
}

impl<'src> GrammarParser<'src> {
	pub fn new(source: Cow<'static, str>) -> Self {
		let mut tokenizer = Tokenizer::new(source);
		tokenizer.scan();
		println!("{:#?}", tokenizer.tokens);
		Self { tokens: BiDirectionalIterator::new(tokenizer.tokens) }
	}
	fn expect(&mut self, rule_name: &str) -> Option<Token> {
		if let Some(next) = self.tokens.peek(0) {
			if next.source == rule_name {
				return self.tokens.next();
			}
		}
		None
	}
	fn expect_one(&mut self, rule_names: Vec<&str>) -> Option<Token> {
		for rule_name in rule_names {
			if let Some(next) = self.expect(rule_name) {
				return Some(next);
			}
		}
		None
	}
	fn expect_constant(&mut self) -> Option<Token> {
		if let Some(next) = self.tokens.peek(0) {
			if is_uppercase(&next.source) {
				return self.tokens.next();
			}
		}
		None
	}
	fn expect_name(&mut self) -> Option<Token> {
		if let Some(next) = self.tokens.peek(0) {
			if is_identifier(&next.source) {
				return self.tokens.next();
			}
		}
		None
	}
	fn expect_string(&mut self) -> Option<Token> {
		if let Some(next) = self.tokens.peek(0) {
			if is_string(&next.source) {
				return self.tokens.next();
			}
		}
		None
	}
	fn expect_tag(&mut self) -> Option<Token> {
		if let Some(next) = self.tokens.peek(0) {
			if is_tag(&next.source) {
				return self.tokens.next();
			}
		}
		None
	}
	pub fn grammar(&mut self) -> Vec<Rule> {
		let mut pos = self.tokens.index();
		let mut rules = vec![];
		while let Some(rule) = self.rule() {
			rules.push(rule);
			pos = self.tokens.index();
			if self.tokens.at_end() {
				break;
			}
		}
		self.tokens.goto(pos);
		rules
	}
	
	fn rule(&mut self) -> Option<Rule> {
		let mut pass_through = false;
		let pos = self.tokens.index();
		if let Some(_dash) = self.expect("-") {
			
			// pass_through modifier
			if let Some(_tilde) = self.expect("~") {
				pass_through = true;
			}
			
			if let Some(name) = self.expect_name() {
				if let Some(_colon) = self.expect(":") {
					let mut alts = vec![self.alternative()];
					let mut alt_pos = self.tokens.index();
					while let Some(_option) = self.expect("|") {
						alts.push(self.alternative());
						alt_pos = self.tokens.index();
					}
					self.tokens.goto(alt_pos);
					return Some(Rule { name: name.source, alts, pass_through });
				}
			}
		}
		self.tokens.goto(pos);
		None
	}
	
	fn group(&mut self) -> Vec<Alt> {
		let mut items = vec![];
		loop {
			if let Some(_paren) = self.expect(")") {
				break;
			}
			if let Some(item) = self.item() {
				items.push(item);
			} else {
				break;
			}
		}
		items
	}
	
	fn alternative(&mut self) -> Vec<AltGroup> {
		let mut items = vec![];
		loop {
			let mut group = AltGroup {
				alts: vec![],
				prefix_modifier: self.prefix_modifier(),
				suffix_modifier: None
			};
			if let Some(_paren) = self.expect("(") {
				group.alts = self.group();
				group.suffix_modifier = self.suffix_modifier();
				items.push(group);
			} else if let Some(item) = self.item() {
				group.suffix_modifier = self.suffix_modifier();
				group.alts.push(item);
				items.push(group);
			} else {
				break;
			}
		}
		items
	}
	
	fn item(&mut self) -> Option<Alt> {
		if let Some(_end) = self.expect("EOF") {
			return None;
		}
		let prefix_modifier = self.prefix_modifier();
		if let Some(con) = self.expect_constant() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "constant".into(), source: con, prefix_modifier, suffix_modifier });
		}
		if let Some(name) = self.expect_name() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "name".into(), source: name, prefix_modifier, suffix_modifier });
		}
		if let Some(string) = self.expect_string() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "string".into(), source: string, prefix_modifier, suffix_modifier });
		}
		if let Some(tag) = self.expect_tag() {
			let suffix_modifier = self.suffix_modifier();
			return Some(Alt { kind: "tag".into(), source: tag, prefix_modifier, suffix_modifier })
		}
		None
	}
	
	fn prefix_modifier(&mut self) -> Option<Cow<'static, str>> {
		if let Some(token) = self.expect_one(vec!["~", "&", "!"]) {
			return Some(token.source);
		}
		None
	}
	
	fn suffix_modifier(&mut self) -> Option<Cow<'static, str>> {
		if let Some(token) = self.expect_one(vec!["?", "*", "+"]) {
			return Some(token.source);
		}
		None
	}
}