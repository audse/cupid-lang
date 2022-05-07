use serde::{Serialize, Deserialize};
use crate::{Expression, Value, Tree, LexicalScope, Token, ErrorHandler};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Array {
	pub items: Vec<Expression>
}

impl Tree for Array {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let items: Vec<Value> = self.items
			.iter()
			.map(|i| i.resolve(scope))
			.collect();
		Value::Array(items)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
	pub inclusive: (bool, bool),
	pub start: Box<Expression>,
	pub end: Box<Expression>,
	pub token: Token,
}

impl Tree for Range {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let start = crate::resolve_or_abort!(self.start, scope);
		let end = crate::resolve_or_abort!(self.end, scope);
		match (start, end) {
			(Value::Integer(s), Value::Integer(e)) => {
				let s = if self.inclusive.0 { s } else { s + 1 };
				let e = if self.inclusive.1 { e + 1 } else { e };
				let a: Vec<Value> = (s..e).map(|i| Value::Integer(i)).collect();
				
				Value::Array(a)
			},
			(x, y) => self.error(format!("start and end of an array must by integers, not {x} and {y}"))
		}
	}
}

impl ErrorHandler for Range {
	fn get_token(&self) -> &Token {
    	&self.token
	}
	fn get_context(&self) -> String {
		format!("creating a range from {} to {}", self.start, self.end)
	}
}

