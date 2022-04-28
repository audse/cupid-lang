use serde::{Serialize, Deserialize};
use crate::{Expression, Value, Tree, LexicalScope};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Array {
	pub items: Vec<Expression>
}

impl Tree for Array {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let items: Vec<Box<Value>> = self.items
			.iter()
			.map(|i| Box::new(i.resolve(scope)))
			.collect();
		Value::Array(items)
	}
}
