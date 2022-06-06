use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Tabled)]
pub struct Closure<'scope> {
	#[tabled(display_with="fmt_option")]
	pub parent: Option<usize>,
	#[tabled(display_with="fmt_vec")]
	pub scopes: Vec<Scope<'scope>>
}

impl Closure<'_> {
	pub fn new(context: Context) -> Self {
		Self {
			parent: None,
			scopes: vec![Scope::new(context)]
		}
	}
	pub fn new_child(parent: usize, context: Context) -> Self {
		Self {
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
