use crate::*;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ParseNode {
	pub name: Cow<'static, str>,
	pub children: Vec<ParseNode>,
	pub tokens: Vec<Token>,
}

impl ParseNode {
	pub fn source(&self) -> Cow<'static, str> {
		self.tokens[0].source.to_owned()
	}
	pub fn token(&self) -> Token {
		self.tokens[0].to_owned()
	}
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
	pub fn child_is(&mut self, index: usize, name: &str) -> bool {
		&*(self.children[index].name) == name
	}
	pub fn collect_tokens(&mut self) -> Vec<Token> {
		self.children.iter_mut().flat_map(|c| {
			let mut tokens = c.tokens.to_owned();
			tokens.append(&mut c.collect_tokens());
			tokens
		}).collect()
	}
}

impl std::fmt::Debug for ParseNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		match (self.children.len() > 0, self.tokens.len() > 0) {
			(true, true) => {
				f.debug_struct(&format!("\"{}\"", self.name))
					.field("tokens", &format!("{}", DisplayVec::new(&self.tokens, false)))
					.field("children", &self.children)
					.finish()
			},
			(true, false) => {
				f.debug_struct(&*self.name)
					.field("children", &self.children)
					.finish()
			},
			(false, true) => {
				f.debug_struct(&*self.name)
					.field("tokens", &format!("{}", DisplayVec::new(&self.tokens, false)))
					.finish()
			},
			(false, false) => {
				f.debug_struct(&*self.name)
					.finish()
			},
		}
	}
}

impl Display for ParseNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{:#?}", self)
	}
}