use crate::*;

#[derive(Debug, Clone)]
pub struct Env {
	pub global: Scope,
	pub closures: Vec<Closure>,
	pub current: usize,
}

impl Env {
	pub fn add_closure(&mut self) -> usize {
		self.closures.push(Closure::new());
		self.closures.len() - 1
	}
	pub fn pop_closure(&mut self) -> Option<Closure> {
		self.closures.pop()
	}
	pub fn add(&mut self, context: Context) {
		self.closures.last_mut().unwrap().add(context);
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.closures.last_mut().unwrap().pop()
	}
}

impl ScopeSearch for Env {
	fn get_symbol(&mut self, symbol: &Ident) -> Option<&SymbolValue> {
		if let Some(closure) = self.closures.get_mut(self.current) {
			if let Some(value) = closure.get_symbol(symbol) {
				return Some(value)
			}
		}
		self.global.get_symbol(symbol)
	}
	fn get_type(&mut self, symbol: &Ident) -> Option<Type> {
		if let Some(closure) = self.closures.get_mut(self.current) {
			if let Some(value) = closure.get_type(symbol) {
				return Some(value)
			}
		}
		self.global.get_type(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		if let Some(closure) = self.closures.get_mut(self.current) {
			closure.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, closure_function: &dyn Fn(&mut SymbolValue)) {
		if let Some(closure) = self.closures.get_mut(self.current) {
			closure.modify_symbol(symbol, closure_function);
		}
	}
}