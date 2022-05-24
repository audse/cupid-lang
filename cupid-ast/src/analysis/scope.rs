use crate::{
	Ident,
	Type,
	ErrCode
};

mod closure;
pub use closure::*;

mod env;
pub use env::*;

mod single_scope;
pub use single_scope::*;

mod symbol_value;
pub use symbol_value::*;


pub trait ScopeSearch {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, ErrCode>;
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, ErrCode>;
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue);
	fn modify_symbol(&mut self, symbol: &Ident, function: &dyn Fn(&mut SymbolValue));
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