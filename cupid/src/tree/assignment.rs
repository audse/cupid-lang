use serde::{Serialize, Deserialize};
use crate::{LexicalScope, Value, Expression, Symbol, Tree, Token, ErrorHandler, SymbolFinder};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assign {
	pub operator: Token,
	pub symbol: Symbol,
	pub value: Box<Expression>
}

impl Tree for Assign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let val = crate::resolve_or_abort!(self.value, scope);
		if let Some(_symbol_type) = scope.get_symbol_type(&self.symbol) {
			match scope.set_symbol(&self.symbol, val.clone()) {
				Ok(result) => match result {
					Some(v) => v,
					None => self.symbol.error_unable_to_assign(&val)
				},
				Err(error) => error
			}
		} else {
			self.symbol.error_unable_to_assign(&val)
		}
	}
}

impl ErrorHandler for Assign {
	fn get_token(&self) -> &Token {
    	&self.operator
	}
	fn get_context(&self) -> String {
    	format!("\n\t  attempting to assign to {} \n\t  value: {}", self.symbol, self.value)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declare {
	pub symbol: Symbol,
	pub value: Box<Expression>,
	pub value_type: Box<Expression>,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Tree for Declare {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	let val = crate::resolve_or_abort!(self.value, scope);
		let symbol_type = crate::resolve_or_abort!(self.value_type, scope);
		if let Value::Type(stored_type) = symbol_type {
			if let Some(value) = scope.create_symbol(
				&self.symbol,
				val.clone(), 
				stored_type, 
				self.mutable, 
				self.deep_mutable
			) {
				return value;
			}
			self.unable_to_assign_error(self.symbol.get_identifier(), val)
		} else {
			self.unable_to_assign_error(self.symbol.get_identifier(), val)
		}
	}
}

impl ErrorHandler for Declare {
	fn get_token(&self) -> &Token {
    	&self.symbol.token
	}
	fn get_context(&self) -> String {
    	format!(
			"\n\t  declaring variable `{}` ({}, {}) \n\t  with value: {}",  
			self.symbol.get_identifier(), 
			self.value_type,
			if self.mutable { "mutable" } else { "immutable" },
			self.value
		)
	}
}