use crate::{Expression, Tree, LexicalScope, Value, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Operator {
	pub operator: Token, 
	pub left: Box<Expression>, 
	pub right: Box<Expression>,
}

impl Operator {
	pub fn new(operator: Token, left: Expression, right: Expression) -> Self {
		Self {
			operator,
			left: Box::new(left),
			right: Box::new(right),
		}
	}
}

impl Tree for Operator {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let left = crate::resolve_or_abort!(self.left, scope);
		let right = crate::resolve_or_abort!(self.right, scope);
		
		let return_val = if left != Value::None {
			 match self.operator.source.as_str() {
				"+" => left + right,
				"-" => left - right,
				"*" => left * right,
				"/" => left / right,
				"is" | "==" => Value::Boolean(left == right),
				"not" | "!=" => Value::Boolean(left != right),
				">" => Value::Boolean(left > right),
				">=" => Value::Boolean(left >= right),
				"<" => Value::Boolean(left < right),
				"<=" => Value::Boolean(left <= right),
				op => Value::error(&self.operator, format!(
					"Unknown binary operator: '{:?}' (evaluating {} {:?} {})", op, left, op, right
				), String::new())
			}
		} else {
			match self.operator.source.as_str() {
				"-" => -right,
				op => Value::error(&self.operator, format!(
					"Unknown unary operator: '{:?}' (evaluating {} {})", op, op, right
				), String::new())
			}
		};
		if return_val == Value::None {
			Value::error(&self.operator, format!(
				"Unable to evaluate: (evaluating {} {:?} {})", self.left, &self.operator, self.right
			), String::new())
		} else {
			return_val
		}
	}
}