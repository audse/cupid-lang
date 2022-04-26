use std::fmt::{Display, Formatter, Result};
// use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use crate::{Value, Tree, LexicalScope, Token, ErrorHandler, SymbolFinder, TypeKind};

#[derive(Debug, Clone)]
pub struct Symbol  {
	pub identifier: Value,
	pub token: Token
}

impl Display for Symbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "identifier `{}`", self.identifier)
	}
}

impl Tree for Symbol {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(value) = scope.get_symbol(self) {
			return value;
		}
		self.error_undefined()
	}
}

impl Symbol {
	pub fn new_string<S>(identifier: S, token: Token) -> Self where S: Into<String> {
		Self {
			identifier: Value::String(identifier.into()), 
			token
		}
	}
	pub fn get_identifier(&self) -> String {
		match &self.identifier {
			Value::String(s) => s.to_string(),
			_ => panic!("no identifier")
		}
	}
	pub fn error_undefined(&self) -> Value {
		self.error(format!("undefined: `{}` does not exist", &self.get_identifier()))
	}
	pub fn error_unable_to_assign(&self, assign_value: &Value) -> Value {
		self.error_context(
			format!("cannot assign {} to {}", assign_value, self.identifier),
			format!("assigning value type {}", TypeKind::infer(assign_value))
		)
	}
	pub fn error_assign_type_mismatch(&self, assign_value: &Value, current_type: TypeKind) -> Value {
		self.error_context(
			format!("type mismatch: variable `{}` is a different type than the given value.", self.identifier),
			format!("\n\t\texpecting: {current_type}\n\t\tfound:     {}", TypeKind::infer(assign_value))
		)
	}
}

impl ErrorHandler for Symbol {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
    	format!("accessing symbol `{}`", self.get_identifier())
	}
}


impl PartialEq for Symbol {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}
impl Eq for Symbol {}

impl Hash for Symbol {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
		self.token.hash(state);
	}
}
