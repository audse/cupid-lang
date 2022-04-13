use crate::{LexicalScope, Value, Expression, Symbol, Tree, Token, Type};

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
				Some(v) => v.clone(),
				None => Value::error(
					&self.operator, 
					format!("unable to assign `{}` to `{}`", self.symbol.0, val)
				)
			},
			Err(error) => error
		}
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Declare {
	pub symbol: Symbol,
	pub value: Box<Expression>,
	pub r#type: Type,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Tree for Declare {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	let val = self.value.resolve(scope);
		if val.is_poisoned() {
			return val;
		}
		if let Some(value) = scope.create_symbol(&self.symbol, val, self.mutable, self.deep_mutable) {
			return value.clone();
		}
		Value::error(&self.symbol.1[0], "unable to declare variable")
	}
}