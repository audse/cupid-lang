use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Closure {
	pub id: usize,
	pub name: Option<Ident>,
	pub parent: Option<usize>,
	pub scopes: Vec<Scope>
}

impl Closure {
	pub fn new(id: usize, name: Option<Ident>, context: Context) -> Self {
		Self {
			id,
			name,
			parent: None,
			scopes: vec![Scope::new(id + 1, context)]
		}
	}
	pub fn new_child(id: usize, name: Option<Ident>, parent: usize, context: Context) -> Self {
		Self {
			id,
			name,
			parent: Some(parent),
			scopes: vec![Scope::new(id + 1, context)]
		}
	}
	pub fn add(&mut self, context: Context) {
		self.scopes.push(Scope { 
			id: self.id + self.scopes.len(), 
			symbols: HashMap::default(),
			context, 
		})
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.scopes.pop()
	}
}
