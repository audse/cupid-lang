use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Tabled)]
pub struct Scope {
	pub context: Context,
	#[tabled(display_with="fmt_map")]
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
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, (Source, ErrCode)> {
		if let Some(value) = self.symbols.get(symbol) {
			Ok(value.to_owned())
		} else {
			Err((symbol.src(), 404))
		}
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, (Source, ErrCode)> {
		if let Ok(value) = self.get_symbol(symbol) {
			if let Ok(value) = value.as_type() {
				return Ok(value);
			}
		}
		Err((symbol.src(), 404))
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.symbols.insert(symbol.to_owned(), value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, mut function: impl FnMut(&mut SymbolValue) -> Result<(), (Source, ErrCode)>) -> Result<(), (Source, ErrCode)> {
		self.symbols.entry(symbol.to_owned()).and_modify(|v| function(v).ok().unwrap());
		Ok(())
	}
}
