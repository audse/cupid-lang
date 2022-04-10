use crate::{Scope, Value, Expression, Symbol, Tree, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Assign {
	pub operator: Token,
	pub symbol: Symbol,
	pub value: Box<Expression>
}

impl Tree for Assign {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let val = self.value.resolve(scope);
		if let Some(value) = scope.set_symbol(&self.symbol, val) {
			return value.clone();
		}
		Value::None
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Declare {
	pub symbol: Symbol,
	pub value: Box<Expression>,
	pub mutable: bool,
	pub deep_mutable: bool,
}

impl Tree for Declare {
	fn resolve(&self, scope: &mut Scope) -> Value {
    	let val = self.value.resolve(scope);
		if let Some(value) = scope.create_symbol(&self.symbol, val, self.mutable, self.deep_mutable) {
			return value.clone();
		}
		Value::None
	}
}