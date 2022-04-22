use crate::{LexicalScope, Value, Expression, Symbol, Tree, Type, ProductType, Token, TypeSymbol, ErrorHandler, SymbolFinder, TypeChecker};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Assign {
	pub operator: Token,
	pub symbol: Symbol,
	pub value: Box<Expression>
}

impl Tree for Assign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let val = crate::resolve_or_abort!(self.value, scope);
		if let Some(symbol_type) = scope.get_symbol_type(&self.symbol) {
			println!("{symbol_type}");
			if scope.can_assign(&val, &symbol_type) {
				match scope.set_symbol(&self.symbol, val.clone()) {
					Ok(result) => match result {
						Some(v) => v,
						None => self.symbol.error_unable_to_assign(&val)
					},
					Err(error) => error
				}
			} else {
				self.symbol.error_assign_type_mismatch(&val, &symbol_type)
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
    	format!("attempting to assign value to {}", self.symbol)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Declare {
	pub symbol: Symbol,
	pub value: Box<Expression>,
	pub r#type: TypeSymbol,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Tree for Declare {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	let val = crate::resolve_or_abort!(self.value, scope);
		if let Value::Type(mut stored_type) = self.r#type.resolve(scope) {
			stored_type.apply_arguments(&self.r#type.arguments);
			
			if scope.can_assign(&val, &stored_type) {
				if let Some(value) = scope.create_symbol_of_type(
					&self.symbol,
					val.clone(), 
					stored_type, 
					self.mutable, 
					self.deep_mutable
				) {
					return value;
				}
			}
			self.r#type.error_assign_type_mismatch(&val, self.symbol.token.clone())
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
			"declaring {} variable {} ({}) with value {}", 
			self.r#type, 
			self.symbol, 
			if self.mutable { "mutable" } else { "immutable" },
			self.value
		)
	}
}