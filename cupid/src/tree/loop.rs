use crate::{
	CupidExpression,
	CupidValue,
	CupidScope,
	Tree,
};

#[derive(Debug, Hash, Clone)]
pub struct CupidLoop {
	pub expressions: Vec<CupidExpression>,
}

impl Tree for CupidLoop {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let mut result = CupidValue::None;
		for exp in &self.expressions {
			result = exp.resolve(scope);
		}
		return result;
	}
}