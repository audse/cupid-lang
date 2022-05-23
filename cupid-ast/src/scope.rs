mod closure;
pub use closure::*;

mod env;
pub use env::*;

mod scope;
pub use scope::*;

mod symbol_value;
pub use symbol_value::*;

pub fn get_symbol_or_panic<'a>(ident: &crate::Ident, scope: &'a mut Env) -> &'a crate::SymbolValue {
	if let Some(value) = scope.get_symbol(ident) {
		value
	} else {
		panic!("symbol could not be found")
	}
}

pub fn get_type_or_panic(ident: &crate::Ident, scope: &mut Env) -> crate::Type {
	if let Some(type_hint) = scope.get_type(ident) {
		type_hint
	} else {
		panic!("type could not be found")
	}
}