use std::fmt::{Display, Formatter, Result};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use crate::{Value, Tree, LexicalScope, Token, ErrorHandler, Type, SymbolFinder};

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
	pub fn new_string(identifier: String, token: Token) -> Self {
		Self {
			identifier: Value::String(identifier), 
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
		self.error(format!("cannot assign {} ({}) to {}", assign_value, Type::from(assign_value), self))
	}
	pub fn error_assign_type_mismatch(&self, assign_value: &Value, current_type: &Type) -> Value {
		self.error(format!(
			"type mismatch: cannot assign {} ({}) to {} ({})", 
			Type::from(assign_value), 
			assign_value, 
			self,
			&current_type
		))
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

#[derive(Debug, Clone)]
pub struct TypeSymbol {
	pub name: Cow<'static, str>, 
	pub arguments: Vec<TypeSymbol>,
	pub token: Option<Token>,
	pub generic: bool,
}

impl Display for TypeSymbol {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "type `{}`", self.name)
	}
}

impl Tree for TypeSymbol {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(type_value) = scope.get_definition(self) {
			Value::Type(type_value)
		} else {
			self.error_undefined()
		}
	}
}

impl TypeSymbol {
	pub fn new<T>(identifier: T, fields: Vec<TypeSymbol>, token: Token, generic: bool) -> Self where T: Into<String> {
		Self { name: Cow::Owned(identifier.into()), arguments: fields, token: Some(token), generic }
	}
	pub const fn new_const(value: &'static str) -> Self {
		Self { name: Cow::Borrowed(value), token: None, arguments: vec![], generic: false }
	}
	pub const fn new_const_generic(value: &'static str) -> Self {
		Self { name: Cow::Borrowed(value), token: None, arguments: vec![], generic: true }
	}
	
	pub fn error(&self, message: String, token: Option<Token>) -> Value {
		if let Some(symbol_token) = &self.token {
			Value::error(symbol_token, message, String::new())
		} else if let Some(error_token) = token {
			Value::error(&error_token, message, String::new())
		} else {
			println!("message {}", message);
			unreachable!()
		}
	}
	pub fn error_undefined(&self) -> Value {
		self.error(format!("undefined: {} does not exist", &self), None)
	}
	pub fn error_assign_type_mismatch(&self, assign_value: &Value, token: Token) -> Value {
		self.error(format!("type mismatch: cannot assign {:#} ({}) to {}", assign_value, Type::from(assign_value), self), Some(token))
	}
}

impl PartialEq for TypeSymbol {
	fn eq(&self, other: &Self) -> bool {
		self.name.eq(&other.name)
	}
}

impl Eq for TypeSymbol {}

impl Hash for TypeSymbol {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.token.hash(state);
	}
}