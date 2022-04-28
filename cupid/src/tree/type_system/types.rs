use serde::{Serialize, Deserialize};
use crate::{Token, Value, Tree, Symbol, LexicalScope, SymbolFinder, ErrorHandler, TypeKind};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefineType {
	pub token: Token,
	pub type_symbol: Symbol,
	pub type_value: TypeKind,
}

impl Tree for DefineType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		if let Some(new_type) = scope.define_type(&self.type_symbol, self.type_value.clone()) {
			new_type
		} else {
			self.error("unable to define")
		}
	}
}

impl ErrorHandler for DefineType {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
    	format!("defining type {} with value {}", self.type_symbol, self.type_value)
	}
}