use std::fmt::{Display, Formatter, Result};
use crate::{Value, Tree, Scope};


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Symbol(pub Value);

impl Display for Symbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:?}", self)
	}
}

impl Tree for Symbol {
	fn resolve(&self, scope: &mut Scope) -> Value {
		if let Some(value) = scope.get_symbol(self) {
			return value.clone();
		}
		Value::None
	}
}

impl Symbol {
	pub fn new(identifier: String) -> Self {
		Self(Value::String(identifier))
	}
}
