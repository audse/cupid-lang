use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use crate::{TypeSymbol, Token, Value, Symbol, Tree, LexicalScope, SymbolFinder};
use super::builtin::*;

#[derive(Debug, Clone)]
pub struct Type {
	pub symbol: TypeSymbol,
	pub fields: Vec<(TypeSymbol, Symbol)>
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.symbol.name == other.symbol.name
	}
}

impl Eq for Type {}

impl Hash for Type {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.symbol.hash(state);
		self.fields.iter().for_each(|(_, s)| s.hash(state));
	}
}

impl Type {
	pub const fn new_const(name: &'static str) -> Self {
		Self {
			symbol: TypeSymbol::new_const(name),
			fields: vec![]
		}
	}
	pub fn from(value: &Value) -> Type {
		match value {
			Value::Boolean(_) => BOOLEAN,
			Value::Integer(_) => INTEGER,
			Value::Decimal(_, _) => DECIMAL,
			Value::String(_) => STRING,
			Value::FunctionBody(_, _) => FUNCTION,
			Value::Dictionary(_) => DICTIONARY,
			Value::List(_) => LIST,
			Value::Tuple(_) => TUPLE,
			Value::MapEntry(_, _) => MAP_ENTRY,
			Value::Error(_) => ERROR,
			_ => NONE
		}
	}
	pub fn is_builtin(&self) -> bool {
		vec![&BOOLEAN, &INTEGER, &DECIMAL, &STRING, &FUNCTION, &DICTIONARY, &LIST, &TUPLE, &NONE, &ERROR].contains(&self)
	}
	pub fn get_name(&self) -> String {
		self.symbol.name.to_string()
	}
	pub fn is(&self, name: &str) -> bool {
		self.symbol.name == name
	}
	// is either a builtin map type or a struct type
	pub fn is_map(&self) -> bool {
		if [DICTIONARY, LIST, TUPLE].contains(self) {
			true
		} else {
			!self.is_builtin()
		}
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.symbol)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineType {
	pub token: Token,
	pub type_value: Type,
}

impl Tree for DefineType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	match scope.define_type(&self.type_value.symbol, self.type_value.clone()) {
			Ok(new_type) => Value::Type(new_type),
			Err(error) => error
		}
	}
}


