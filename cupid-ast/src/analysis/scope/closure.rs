use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Closure {
	pub parent: Option<usize>,
	pub scopes: Vec<Scope>
}

impl Closure {
	pub fn new() -> Self {
		Self { 
			parent: None,
			scopes: vec![Scope::new(Context::Closure)] 
		}
	}
	pub fn new_child(parent: usize) -> Self {
		Self {
			parent: Some(parent),
			scopes: vec![Scope::new(Context::Closure)]
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
impl ScopeSearch for Closure {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ErrCode> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value);
			}
		}
		Err(404)
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ErrCode> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_type(symbol) {
				return Ok(value);
			}
		}
		Err(404)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.scopes.last_mut().unwrap().set_symbol(symbol, value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: &dyn Fn(&mut SymbolValue)) {
		let mut container: Option<&mut Scope> = None;
		for scope in self.scopes.iter_mut() {
			if scope.get_symbol(symbol).is_ok() {
				container = Some(scope);
			}
		}
		if let Some(container) = container {
			container.modify_symbol(symbol, function);
		} else {
			panic!("symbol not found to modify")
		}
	}
}