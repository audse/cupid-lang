use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
	pub context: Context,
	pub symbols: HashMap<Ident, SymbolValue>,
}

impl Scope {
	pub fn new(context: Context) -> Self {
		Self {
			context,
			symbols: HashMap::default()
		}
	}
}

impl ScopeSearch for Scope {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ErrCode> {
		if let Some(value) = self.symbols.get(symbol) {
			Ok(value.to_owned())
		} else {
			Err(404)
		}
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ErrCode> {
		if let Ok(value) = self.get_symbol(symbol) {
			if let Ok(value) = value.as_type() {
				return Ok(value);
			}
		}
		Err(404)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.symbols.insert(symbol.to_owned(), value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: &dyn Fn(&mut SymbolValue)) {
		self.symbols.entry(symbol.to_owned()).and_modify(function);
	}
}
