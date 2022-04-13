use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use crate::{Value, Tree, LexicalScope, Token, Scope};


#[derive(Debug, Clone)]
pub struct Symbol(pub Value, pub Vec<Token>, pub Scope);

impl Display for Symbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "identifier `{}`", self.0)
	}
}

impl Tree for Symbol {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(value) = scope.get_symbol(self) {
			return value;
		}
		Value::error(&self.1[0], format!("identifier `{}` is not defined", self.get_identifier()))
	}
}

impl Symbol {
	pub fn new(identifier: String, tokens: Vec<Token>) -> Self {
		Self(Value::String(identifier), tokens, Scope::new(None))
	}
	pub fn get_identifier(&self) -> String {
		match &self.0 {
			Value::String(s) => s.to_string(),
			_ => panic!("no identifier")
		}
	}
}

impl PartialEq for Symbol {
	fn eq(&self, other: &Self) -> bool {
    	self.0 == other.0
	}
}

impl Eq for Symbol {}

impl Hash for Symbol {
	fn hash<H: Hasher>(&self, _: &mut H) {}
}