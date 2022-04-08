use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	CupidExpression,
	Operator,
	Tree,
	CupidScope,
	CupidValue,
};

#[derive(Debug, Hash, Clone)]
pub struct CupidOperator {
	pub operator: Operator, 
	pub left: Box<CupidExpression>, 
	pub right: Box<CupidExpression>
}

impl Display for CupidOperator {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:?}", self)
	}
}

impl CupidOperator {
	pub fn new(operator: Operator, left: CupidExpression, right: CupidExpression) -> Self {
		Self {
			operator,
			left: Box::new(left),
			right: Box::new(right)
		}
	}
}

impl Tree for CupidOperator {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let left = self.left.resolve(scope);
		let right = self.right.resolve(scope);
		println!("Resolving l {}, r {}", left, right);
		
		return CupidValue::op(self.operator, &left, &right);
	}
}