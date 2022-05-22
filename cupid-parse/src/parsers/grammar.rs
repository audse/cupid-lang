use crate::*;

pub struct GrammarParser {
	pub name: Str,
	pub tokens: BiDirectionalIterator<Token>,
	pub file: usize,
}

impl Parser for GrammarParser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
		&mut self.tokens
	}
	fn file(&self) -> usize { self.file }
}

impl GrammarParser {
	pub fn new(name: Str, source: Cow<'static, str>, file: usize) -> Self {
		Self { name, tokens: Self::build(source.into(), file), file }
	}
	
	pub fn grammar(&mut self) -> Grammar {
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
		Grammar {
			name: self.name.to_owned(),
			rules,
		}
	}
	
	// fn dependencies(&mut self) -> Vec<Str> {
	// 	let mut dependencies = vec![];
	// 	while let Some(_) = self.expect("use") {
	// 		if let Some((name, _)) = self.expect_word() {
	// 			dependencies.push(name.source())
	// 		}
	// 	}
	// 	dependencies
	// }
	
	fn rule(&mut self) -> Option<Rule> {
		let mut pass_through = false;
		let mut inverse = false;
		let pos = self.tokens.index();
		if let Some(_) = self.expect("-") {
			
			// pass_through modifier
			if let Some(_) = self.expect("~") {
				pass_through = true;
			}
			if let Some(_) = self.expect("!") {
				inverse = true;
			}
			if let Some((name, _)) = self.expect_word() {
				if let Some(_) = self.expect(":") {
					let alts = self.rule_body();
					return Some(Rule { 
						name: name.source(), 
						alts, 
						pass_through,
						inverse,
						params: vec![],
					});
				} else {
					let (params, alts) = self.function_rule();
					return Some(Rule {
						name: name.source(),
						alts,
						pass_through,
						inverse,
						params,
					})
				}
			}
		}
		self.tokens.goto(pos);
		None
	}
	
	fn rule_body(&mut self) -> Vec<Vec<StaticGroup>> {
		let mut alts = vec![self.alternative()];
		let mut alt_pos = self.tokens.index();
		while let Some(_) = self.expect("|") {
			alts.push(self.alternative());
			alt_pos = self.tokens.index();
		}
		self.tokens.goto(alt_pos);
		alts
	}
	
	fn function_rule(&mut self) -> (Vec<Str>, Vec<Vec<StaticGroup>>) {
		if let Some(_) = self.expect("[") {
			let params = self.params();
			if let Some(_) = self.expect(":") {
				return (params, self.rule_body());
			}
		}
		(vec![], vec![])
	}
	
	fn params(&mut self) -> Vec<Str> {
		let mut args = vec![];
		while let Some((arg, _)) = self.expect_word() {
			args.push(arg.tokens[0].source.to_owned());
			self.expect(",");
			if let Some(_) = self.expect("]") {
				break;
			}
		}
		args
	}
	
	fn group(&mut self) -> Vec<StaticItem> {
		let mut items = vec![];
		loop {
			if let Some(_) = self.expect(")") {
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
	
	fn alternative(&mut self) -> Vec<StaticGroup> {
		let mut items: Vec<StaticGroup> = vec![];
		loop {
			let mut group = Group {
				items: vec![].into(),
				prefix_modifier: self.prefix_modifier(),
				suffix_modifier: None
			};
			if let Some(_) = self.expect("(") {
				group.items = self.group();
				group.suffix_modifier = self.suffix_modifier();
				items.push(Cow::Owned(group));
			} else if let Some(item) = self.item() {
				group.suffix_modifier = self.suffix_modifier();
				group.items.push(item);
				items.push(Cow::Owned(group));
			} else {
				break;
			}
		}
		items
	}
	
	fn make_item(&mut self, kind: &'static str, node: ParseNode, prefix_modifier: Option<Str>) -> StaticItem {
		let args = self.args();
		let suffix_modifier = self.suffix_modifier();
		Cow::Owned(Item { 
			kind: kind.into(), 
			source: node.token(), 
			prefix_modifier, 
			suffix_modifier,
			args 
		})
	}
	
	fn item(&mut self) -> Option<StaticItem> {
		if let Some(_) = self.expect("EOF") {
			return None;
		}
		let prefix = self.prefix_modifier();
		if let Some((con, _)) = self.expect_constant() {
			return Some(self.make_item("constant", con, prefix));
		}
		if let Some((name, _)) = self.expect_word() {
			return Some(self.make_item("name", name, prefix));
		}
		if let Some((string, _)) = self.expect_string() {
			return Some(self.make_item("string", string, prefix));
		}
		if let Some((tag, _)) = self.expect_any_tag() {
			return Some(self.make_item("tag", tag, prefix))
		}
		None
	}
	
	fn args(&mut self) -> Vec<StaticItem> {
		let mut args = vec![];
		if let Some(_) = self.expect("[") {
			while let Some(arg) = self.item() {
				args.push(arg);
				self.expect(",");
				if let Some(_) = self.expect("]") {
					break;
				}
			}
		}
		args
	}
	
	fn prefix_modifier(&mut self) -> Option<Str> {
		if let Some((token_a, _)) = self.expect_one(vec!["="]) {
			if let Some((token_b, _)) = self.expect_one(vec!["&", "!"]) {
				return Some((token_a.source().to_string() + &token_b.source()).into());
			}
		}
		if let Some((token, _)) = self.expect_one(vec!["~", "&", "!"]) {
			return Some(token.source());
		}
		None
	}
	
	fn suffix_modifier(&mut self) -> Option<Str> {
		if let Some((token, _)) = self.expect_one(vec!["?", "*", "+"]) {
			return Some(token.source());
		}
		None
	}
}