use crate::{
	CupidScope,
	CupidValue,
	CupidExpression,
	Assign,
	CupidSymbol,
	Tree,
};

#[derive(Debug, Hash, Clone)]
pub struct CupidAssign {
	pub operator: Assign,
	pub symbol: CupidSymbol,
	pub value: Box<CupidExpression>
}

impl Tree for CupidAssign {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let val = self.value.resolve(scope);
		if let Some(value) = scope.set_symbol(&self.symbol, val) {
			return value.clone();
		}
		return CupidValue::None;
	}
}

#[derive(Debug, Hash, Clone)]
pub struct CupidDeclare {
	pub symbol: CupidSymbol,
	pub value: Box<CupidExpression>,
}

impl Tree for CupidDeclare {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
    	let val = self.value.resolve(scope);
		if let Some(value) = scope.set_symbol(&self.symbol, val) {
			return value.clone();
		}
		return CupidValue::None;
	}
}