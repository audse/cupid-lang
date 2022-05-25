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
	pub fn take_token(&self, index: usize) -> Token {
		self.tokens[index].to_owned()
	}
	pub fn get_map<R, E>(&mut self, name: &str, function: impl FnOnce(&mut Self) -> Result<R, E> ) -> Result<R, E> {
		function(self.get_mut(name).unwrap())
	}
	pub fn option_map<R, E>(&mut self, name: &str, function: impl FnOnce(&mut Self) -> Result<R, E> ) -> Result<Option<R>, E> {
		if let Some(node) = self.get_mut(name) {
			Ok(Some(function(node)?))
		} else {
			Ok(None)
		}
	}
	pub fn map<R, E>(&mut self, function: impl FnMut(&mut Self) -> Result<R, E>) -> Result<Vec<R>, E> {
		self.children.iter_mut().map(function).collect()
	}
	pub fn filter_map<R, E>(&mut self, function: impl FnMut(&mut Self) -> Option<Result<R, E>>) -> Result<Vec<R>, E> {
		self.children.iter_mut().filter_map(function).collect()
	}
	pub fn filter_map_noresult<R>(&mut self, function: &dyn Fn(&mut Self) -> Option<R>) -> Vec<R> {
		self.children.iter_mut().filter_map(function).collect()
	}
	pub fn map_named<R, E>(&mut self, name: &str, mut function: impl FnMut(&mut Self) -> Result<R, E>) -> Result<Vec<R>, E> {
		self.children
			.iter_mut()
			.filter_map(|c| if &*c.name == name { 
				Some(function(c)) 
			} else { 
				None 
			})
			.collect()
	}
	pub fn has(&self, name: &str) -> bool {
		self.children.iter().find(|c| c.name == name).is_some()
	}
	pub fn has_token(&self, name: &str) -> bool {
		self.tokens.iter().find(|c| c.source == name).is_some()
	}
	pub fn get(&mut self, name: &str) -> &mut Self {
		self.children.iter_mut().find(|c| c.name == name).unwrap()
	}
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Self> {
		self.children.iter_mut().find(|c| c.name == name)
	}
	pub fn get_all(&mut self, name: &str) -> Vec<&mut Self> {
		self.children.iter_mut().filter(|c| c.name == name).collect()
	}
	pub fn child_is(&mut self, index: usize, name: &str) -> bool {
		&*(self.children[index].name) == name
	}
	pub fn get_children(&mut self, name: &str) -> Vec<&mut Self> {
		self.get(name).children.iter_mut().collect()
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