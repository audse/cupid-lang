use std::fmt::{Display, Formatter, Result};
use crate::{Expression, Value, Scope, Tree};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Block {
	pub expressions: Vec<Expression>,
}

impl Tree for Block {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let mut result = Value::None;
		for exp in &self.expressions {
			result = exp.resolve(scope);
		}
		result
	}
}

impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Block: {:?}", self.expressions)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct IfBlock {
	pub condition: Box<Expression>,
	pub body: Block,
	pub else_if_bodies: Vec<(Expression, Block)>,
	pub else_body: Option<Block>,
}

impl Tree for IfBlock {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let condition = self.condition.resolve(scope);
		match condition {
			Value::Boolean(b) => if b {
				self.body.resolve(scope)
			} else if self.else_body.is_some() {
				self.else_body.as_ref().unwrap().resolve(scope)
			} else { Value::None },
			_ => Value::None
		}
	}
}