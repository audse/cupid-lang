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
		let left = self.left.resolve(scope);
		let right = self.right.resolve(scope);
		
		if left.is_poisoned() {
			return left;
		}
		if right.is_poisoned() {
			return right;
		}
		if left != Value::None {
			match self.operator.source.as_str() {
				"+" => left.add(right, &self.operator),
				"-" => left.subtract(right, &self.operator),
				"*" => left.multiply(right, &self.operator),
				"/" => left.divide(right, &self.operator),
				"is" => left.equal(&right),
				"not" => left.not_equal(&right),
				">" => left.greater(right, &self.operator),
				">=" => left.greater_equal(right, &self.operator),
				"<" => left.less(right, &self.operator),
				"<=" => left.less_equal(right, &self.operator),
				op => Value::error(&self.operator, format!(
					"Unknown binary operator: '{:?}' (evaluating {} {:?} {})", op, left, op, right
				), String::new())
			}
		} else {
			match self.operator.source.as_str() {
				"-" => right.negative(&self.operator),
				op => Value::error(&self.operator, format!(
					"Unknown unary operator: '{:?}' (evaluating {} {})", op, op, right
				), String::new())
			}
		}
	}
}