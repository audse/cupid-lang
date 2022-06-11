use crate::*;

pub trait UseScope {
	fn get_symbol(&mut self, symbol: &Ident) -> ASTResult<SymbolValue>;
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue);
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> ASTResult<()>) -> ASTResult<()>;
}

impl UseScope for Env {
	fn get_symbol(&mut self, symbol: &Ident) -> ASTResult<SymbolValue> {
		self.trace_get_symbol(symbol);
		if let Ok(value) = self.get_symbol_from(symbol, self.current_closure) {
			return Ok(value);
		}
		self.global.get_symbol(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.trace_set_symbol(symbol, &value);
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.set_symbol(symbol, value);
		} else {
			self.global.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> ASTResult<()>) -> ASTResult<()> {
		self.trace_modify_symbol(symbol);
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.modify_symbol(symbol, function)
		} else {
			self.global.modify_symbol(symbol, function)
		}
	}
}

impl UseScope for Closure {
	fn get_symbol(&mut self, symbol: &Ident) -> ASTResult<SymbolValue> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value);
			}
		}
		
		symbol.to_err(ERR_NOT_FOUND)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.scopes.last_mut().unwrap().set_symbol(symbol, value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> ASTResult<()>) -> ASTResult<()> {
		let mut container: Option<&mut Scope> = None;
		for scope in self.scopes.iter_mut() {
			if scope.get_symbol(symbol).is_ok() {
				container = Some(scope);
			}
		}
		if let Some(container) = container {
			container.modify_symbol(symbol, function)
		} else {
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
}

impl UseScope for Scope {
	fn get_symbol(&mut self, symbol: &Ident) -> ASTResult<SymbolValue> {
		if let Some(value) = self.symbols.get(symbol) {
			Ok(value.to_owned())
		} else {
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.symbols.insert(symbol.to_owned(), value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, mut function: impl FnMut(&mut SymbolValue) -> ASTResult<()>) -> ASTResult<()> {
		let entry = self.symbols.get_mut(symbol);
		if let Some(entry) = entry {
			function(entry)?;
		}
		Ok(())
	}
}