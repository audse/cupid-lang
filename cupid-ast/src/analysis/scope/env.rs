use crate::*;

#[derive(Debug, Clone)]
pub struct Env {
	pub global: Scope,
	pub closures: Vec<Closure>,
	pub current_closure: usize,
	pub prev_closure: Option<usize>,
	pub source_data: Vec<ParseNode>,
}

impl Default for Env {
	fn default() -> Self {
    	Self {
			global: Scope::new(Context::Global),
			closures: vec![Closure { 
				parent: None, 
				scopes: vec![Scope::new(Context::Block)] 
			}],
			current_closure: 0,
			prev_closure: None,
			source_data: vec![]
		}
	}
}

impl Env {
	pub fn add_source(&mut self, source: &mut ParseNode) -> usize {
		self.source_data.push(source.to_owned());
		self.source_data.len() - 1
	}
	pub fn use_closure(&mut self, closure: usize) {
		self.prev_closure = Some(self.current_closure);
		self.current_closure = closure;
	}
	pub fn reset_closure(&mut self) {
		self.current_closure = self.prev_closure.unwrap();
		self.prev_closure = None;
	}
	pub fn add_closure(&mut self) -> usize {
		if let Some(closure_index) = self.prev_closure {
			self.closures.push(Closure::new_child(closure_index));
		} else {
			self.closures.push(Closure::new());
		}
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
	pub fn has_symbol(&mut self, symbol: &Ident) -> Result<(), ErrCode> {
		if matches!(self.get_symbol(symbol), Ok(_)) {
			Ok(())
		} else {
			Err(404)
		}
	}
	pub fn no_symbol(&mut self, symbol: &Ident) -> Result<(), ErrCode> {
		if matches!(self.get_symbol(symbol), Ok(_)) {
			Err(500)
		} else {
			Ok(())
		}
	}
	pub fn get_symbol_from(&mut self, symbol: &Ident, closure_index: usize) -> Result<SymbolValue, ErrCode> {
		let closure = &mut self.closures[closure_index];
		let parent = closure.parent();
		if let Ok(value) = closure.get_symbol(symbol) {
			return Ok(value);
		}
		if let Some(parent_index) = parent {
			self.get_symbol_from(symbol, parent_index)
		} else {
			Err(404)
		}
	}
}

impl ScopeSearch for Env {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ErrCode> {
		if let Ok(value) = self.get_symbol_from(symbol, self.current_closure) {
			return Ok(value);
		}
		self.global.get_symbol(symbol)
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ErrCode> {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			if let Ok(value) = closure.get_type(symbol) {
				return Ok(value)
			}
		}
		self.global.get_type(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: &dyn Fn(&mut SymbolValue)) {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.modify_symbol(symbol, function);
		}
	}
}