use crate::{Expression, Value, LexicalScope, Tree};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Block {
	pub expressions: Vec<Expression>,
}

impl Tree for Block {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		scope.add();
		let mut result = Value::None;
		for exp in &self.expressions {
			result = exp.resolve(scope);
			crate::abort_on_error!(result);
		}
		scope.pop();
		result
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
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
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