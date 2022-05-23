use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Closure {
	scopes: Vec<Scope>
}

impl Closure {
	pub fn new() -> Self {
		Self { scopes: vec![Scope::new(Context::Closure)] }
	}
}

impl Closure {
	pub fn add(&mut self, context: Context) {
		self.scopes.push(Scope { context, symbols: HashMap::default() })
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.scopes.pop()
	}
}
impl ScopeSearch for Closure {
	fn get_symbol(&mut self, symbol: &Ident) -> Option<&SymbolValue> {
		for scope in self.scopes.iter_mut() {
			if let Some(value) = scope.get_symbol(symbol) {
				return Some(value);
			}
		}
		None
	}
	fn get_type(&mut self, symbol: &Ident) -> Option<Type> {
		for scope in self.scopes.iter_mut() {
			if let Some(value) = scope.get_type(symbol) {
				return Some(value);
			}
		}
		None
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.scopes.last_mut().unwrap().set_symbol(symbol, value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, closure: &dyn Fn(&mut SymbolValue)) {
		let container: Option<&mut Scope> = None;
		for scope in self.scopes.iter_mut() {
			if let Some(_) = scope.get_symbol(symbol) {
				container = Some(scope);
			}
		}
		if let Some(container) = container {
			container.modify_symbol(symbol, closure);
		} else {
			panic!("symbol not found to modify")
		}
	}
}