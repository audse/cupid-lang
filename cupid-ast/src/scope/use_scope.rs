use crate::*;

pub trait UseScope {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ASTErr>;
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ASTErr>;
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue);
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> Result<(), ASTErr>) -> Result<(), ASTErr>;
}

impl UseScope for Env {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ASTErr> {
		self.traceback.push(format!("Getting symbol: {symbol}"));
		if let Ok(value) = self.get_symbol_from(symbol, self.current_closure) {
			return Ok(value);
		}
		self.global.get_symbol(symbol)
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ASTErr> {
		self.traceback.push(format!("Getting type symbol: {symbol}"));
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			if let Ok(value) = closure.1.get_type(symbol) {
				return Ok(value)
			}
		}
		self.global.get_type(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.traceback.push(format!("Setting symbol: {symbol} to value: {value}"));
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.1.set_symbol(symbol, value);
		} else {
			self.global.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> Result<(), ASTErr>) -> Result<(), ASTErr> {
		self.traceback.push(format!("Modifying symbol: {symbol}"));
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.1.modify_symbol(symbol, function)
		} else {
			self.global.modify_symbol(symbol, function)
		}
	}
}

impl UseScope for Closure {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ASTErr> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_symbol(symbol) {
				return Ok(value);
			}
		}
		Err((symbol.src(), ERR_NOT_FOUND))
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ASTErr> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_type(symbol) {
				return Ok(value);
			}
		}
		Err((symbol.src(), ERR_NOT_FOUND))
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.scopes.last_mut().unwrap().set_symbol(symbol, value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> Result<(), ASTErr>) -> Result<(), ASTErr> {
		let mut container: Option<&mut Scope> = None;
		for scope in self.scopes.iter_mut() {
			if scope.get_symbol(symbol).is_ok() {
				container = Some(scope);
			}
		}
		if let Some(container) = container {
			container.modify_symbol(symbol, function)
		} else {
			Err((symbol.src(), ERR_NOT_FOUND))
		}
	}
}

impl UseScope for Scope {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ASTErr> {
		if let Some(value) = self.symbols.get(symbol) {
			Ok(value.to_owned())
		} else {
			Err((symbol.src(), ERR_NOT_FOUND))
		}
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ASTErr> {
		if let Ok(value) = self.get_symbol(symbol) {
			if let Ok(value) = value.as_type() {
				return Ok(value);
			}
		}
		Err((symbol.src(), ERR_NOT_FOUND))
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.symbols.insert(symbol.to_owned(), value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, mut function: impl FnMut(&mut SymbolValue) -> Result<(), ASTErr>) -> Result<(), ASTErr> {
		self.symbols.entry(symbol.to_owned()).and_modify(|v| function(v).ok().unwrap());
		Ok(())
	}
}
