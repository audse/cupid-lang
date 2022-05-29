use crate::*;

mod closure;
pub use closure::*;

mod env;
pub use env::*;

mod single_scope;
pub use single_scope::*;

mod symbol_value;
pub use symbol_value::*;


pub trait ScopeSearch {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, (Source, ErrCode)>;
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, (Source, ErrCode)>;
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue);
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> Result<(), (Source, ErrCode)>) -> Result<(), (Source, ErrCode)>;
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Tabled)]
pub enum Context {
	Global,
	Closure,
	Block,
	Loop,
	Type,
	Trait,
	Method,
	Function,
}

impl Default for Context {
	fn default() -> Self {
		Self::Block
	}
}