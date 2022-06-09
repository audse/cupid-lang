use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Tabled)]
pub struct Closure {
	#[tabled(display_with="fmt_option")]
	pub name: Option<Ident>,
	#[tabled(display_with="fmt_option")]
	pub parent: Option<usize>,
	#[tabled(display_with="fmt_vec")]
	pub scopes: Vec<Scope>
}

impl Closure {
	pub fn new(name: Option<Ident>, context: Context) -> Self {
		Self {
			name,
			parent: None,
			scopes: vec![Scope::new(context)]
		}
	}
	pub fn new_child(name: Option<Ident>, parent: usize, context: Context) -> Self {
		Self {
			name,
			parent: Some(parent),
			scopes: vec![Scope::new(context)]
		}
	}
	pub fn add(&mut self, context: Context) {
		self.scopes.push(Scope { context, symbols: HashMap::default() })
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.scopes.pop()
	}
	pub fn parent(&mut self) -> Option<usize> {
		self.parent
	}
}
