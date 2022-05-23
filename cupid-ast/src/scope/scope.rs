use crate::*;

pub trait ScopeSearch {
	fn get_symbol(&mut self, symbol: &Ident) -> Option<&SymbolValue>;
	fn get_type(&mut self, symbol: &Ident) -> Option<Type>;
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue);
	fn modify_symbol(&mut self, symbol: &Ident, closure: &dyn Fn(&mut SymbolValue));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
	Global,
	Closure,
	Block,
	Loop,
}

impl Default for Context {
	fn default() -> Self {
		Self::Block
	}
}

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
	fn get_symbol(&mut self, symbol: &Ident) -> Option<&SymbolValue> {
		self.symbols.get(symbol)
	}
	fn get_type(&mut self, symbol: &Ident) -> Option<Type> {
		if let Some(value) = self.get_symbol(symbol) {
			value.as_type()
		} else {
			None
		}
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		self.symbols.insert(symbol.to_owned(), value);
	}
	fn modify_symbol(&mut self, symbol: &Ident, closure: &dyn Fn(&mut SymbolValue)) {
		self.symbols.entry(symbol.to_owned()).and_modify(closure);
	}
}
