use crate::symbol_table::Address;
use crate::*;

impl Env {
	pub fn get_symbol(&mut self, symbol: &Ident) -> ASTResult<SymbolValue> {
		let address = self.get_address(symbol)?;
		self.symbols
			.get_symbol(address)
			.map(|v| v.to_owned())
			.ok_or_else(|| symbol.as_err(ERR_NOT_FOUND))
	}
	pub fn get_type(&mut self, symbol: &Ident) -> ASTResult<Type> {
		self.trace(format!("getting type {symbol:?}"));
		let address = self.get_address(symbol)?;
		let value = self.symbols
			.get_symbol(address)
			.ok_or_else(|| symbol.as_err(ERR_NOT_FOUND))?
			.to_owned();
		self.trace(format!("Found {value:?}"));
		if let Some(VType(typ)) = value.value {
			return Ok(typ)
		}
		value.to_err(ERR_EXPECTED_TYPE)
	}
	pub fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) -> ASTResult<Address> {
		let address = self.set_address(symbol)?;
		self.symbols.set_symbol(address, value);
		Ok(address)
	}
	pub fn get_address(&mut self, symbol: &Ident) -> ASTResult<Address> {
		if let Ok(value) = self.get_address_from(symbol, self.current_closure) {
			return Ok(value);
		}
		self.global.get_address(symbol)
	}
	pub fn set_address(&mut self, symbol: &Ident) -> ASTResult<Address> {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.set_address(symbol)
		} else {
			Ok(self.global.set_address(symbol))
		}
	}
}

impl Closure {
	pub fn get_address(&mut self, symbol: &Ident) -> ASTResult<Address> {
		for scope in self.scopes.iter_mut() {
			if let Ok(value) = scope.get_address(symbol) {
				return Ok(value);
			}
		}
		symbol.to_err(ERR_NOT_FOUND)
	}
	pub fn set_address(&mut self, symbol: &Ident) -> ASTResult<Address> {
		match self.scopes.last_mut() {
			Some(scope) => Ok(scope.set_address(symbol)),
			None => symbol.to_err(ERR_UNREACHABLE)
		}
	}
}

impl Scope {
	pub fn get_address(&mut self, symbol: &Ident) -> ASTResult<Address> {
		match self.symbols.get(symbol) {
			Some(address) => Ok(*address),
			None => symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn set_address(&mut self, symbol: &Ident) -> Address {
		let address = self.symbols.len() + self.id;
		self.symbols.insert(symbol.to_owned(), address);
		address
	}
}