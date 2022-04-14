use crate::{LexicalScope, Value, Expression, Symbol, Tree, Token, TypeSymbol, is_type};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Assign {
	pub operator: Token,
	pub symbol: Symbol,
	pub value: Box<Expression>
}

impl Tree for Assign {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let val = self.value.resolve(scope);
		if val.is_poisoned() {
			return val;
		}
		match scope.set_symbol(&self.symbol, val.clone()) {
			Ok(result) => match result {
				Some(v) => v,
				None => self.symbol.error_unable_to_assign(&val)
			},
			Err(error) => error
		}
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
    	let val = self.value.resolve(scope);
		if val.is_poisoned() {
			return val;
		}
		if let Value::Type(type_value) = self.r#type.resolve(scope) {
			if is_type(&val, &type_value) {
				if let Some(value) = scope.create_symbol_of_type(
					&self.symbol,
					val.clone(), 
					type_value, 
					self.mutable, 
					self.deep_mutable
				) {
					return value;
				}
			}
			self.r#type.error_assign_type_mismatch(&val, self.symbol.token.clone())
		} else {
			self.r#type.error_undefined()
		}
	}
}