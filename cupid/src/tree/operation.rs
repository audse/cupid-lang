use std::fmt::{Display, Formatter, Result};
use crate::{Expression, Tree, Scope, Value, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Operator {
	pub operator: Token, 
	pub left: Box<Expression>, 
	pub right: Box<Expression>
}

impl Display for Operator {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:?}", self)
	}
}

impl Operator {
	pub fn new(operator: Token, left: Expression, right: Expression) -> Self {
		Self {
			operator,
			left: Box::new(left),
			right: Box::new(right)
		}
	}
}

impl Tree for Operator {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let left = self.left.resolve(scope);
		let right = self.right.resolve(scope);
		
		if left != Value::None {
			match self.operator.source.as_str() {
				"+" => left.add(right),
				"-" => left.subtract(right),
				"*" => left.multiply(right),
				"/" => left.divide(right),
				"is" => left.equal(&right),
				"not" => left.not_equal(&right),
				">" => left.greater(right),
				">=" => left.greater_equal(right),
				"<" => left.less(right),
				"<=" => left.less_equal(right),
				op => panic!("Unknown binary operator: {:?}", op)
			}
		} else {
			match self.operator.source.as_str() {
				"-" => right.negative(),
				op => panic!("Unknown unary operator: {:?}", op)
			}
		}
	}
}