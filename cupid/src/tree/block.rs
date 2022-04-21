use crate::{Expression, Value, LexicalScope, Tree, ScopeContext};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Block {
	pub expressions: Vec<Expression>,
}

impl Tree for Block {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		scope.add(ScopeContext::Block);
		let mut result = Value::None;
		
		for exp in &self.expressions {
			result = crate::resolve_or_abort!(exp, scope);
			if let Expression::Break(_) = exp {
				break
			}
			if let Value::Break(_) = result {
				break
			}
			if let Value::Return(_) = result {
				if scope.within_function() {
					break
				}
			}
			if let Value::Continue = result {
				break
			}
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

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BoxBlock {
	pub expressions: Vec<Expression>,
}

impl Tree for BoxBlock {
	fn resolve(&self, _scope: &mut LexicalScope) -> Value {
		let mut inner_scope = LexicalScope::new(ScopeContext::Boxed);
		let mut result = Value::None;
		for exp in &self.expressions {
			result = crate::resolve_or_abort!(exp, &mut inner_scope);
		}
		inner_scope.pop();
		result
	}
}