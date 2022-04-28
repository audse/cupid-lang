use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{Expression, Value, Tree, LexicalScope, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Map {
	pub items: Vec<(Expression, Expression)>,
	pub token: Token,
}

impl Tree for Map {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let items = self.items
			.iter()
			.enumerate()
			.map(|(i, (k, v))| {
				let key = if let Expression::Symbol(symbol) = k {
					symbol.identifier.clone()
				} else {
					k.resolve(scope)
				};
				let value = v.resolve(scope);
				(key, (i, value)) 
			});
		Value::Map(HashMap::from_iter(items))
	}
}