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
		let condition = crate::resolve_or_abort!(self.condition, scope);
		let has_else_if = self.else_if_bodies.len() > 0;
		let has_else = self.else_body.is_some();
		match condition {
			Value::Boolean(b) => {
				if b {
					return crate::resolve_or_abort!(self.body, scope)
				}
				if has_else_if {
					for body in &self.else_if_bodies {
						if let Value::Boolean(condition) = crate::resolve_or_abort!(body.0, scope) {
							if condition {
								return crate::resolve_or_abort!(body.1, scope);
							}
						}
					}
				}
				if has_else {
					return crate::resolve_or_abort!(self.else_body.as_ref().unwrap(), scope);
				}
				Value::None
			},
			_ => Value::None
		}
	}
}