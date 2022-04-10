use crate::{Expression, Value, Scope, Tree};

#[derive(Debug, Hash, Clone)]
pub struct Loop {
	pub block: Block,
}

impl Tree for Loop {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let mut result = Value::None;
		for exp in &self.block {
			result = exp.resolve(scope);
		}
		return result;
	}
}