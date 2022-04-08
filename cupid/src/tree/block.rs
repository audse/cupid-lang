use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	CupidExpression,
	CupidValue,
	CupidScope,
	Tree,
};

#[derive(Debug, Hash, Clone)]
pub struct CupidBlock {
	pub expressions: Vec<CupidExpression>,
}

impl Tree for CupidBlock {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let mut result = CupidValue::None;
		for exp in &self.expressions {
			result = exp.resolve(scope);
		}
		return result;
	}
}

impl Display for CupidBlock {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Block: {:?}", self.expressions)
	}
}

#[derive(Debug, Hash, Clone)]
pub struct CupidIfBlock {
	pub condition: Box<CupidExpression>,
	pub body: CupidBlock,
	pub else_body: Option<CupidBlock>,
}

impl Tree for CupidIfBlock {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let condition = self.condition.resolve(scope);
		match condition {
			CupidValue::Boolean(b) => if b {
				return self.body.resolve(scope);
			} else if self.else_body.is_some() {
				return self.else_body.as_ref().unwrap().resolve(scope);
			} else { return CupidValue::None },
			_ => return CupidValue::None
		}
	}
}