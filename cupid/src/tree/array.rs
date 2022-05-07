use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArrayNode {
	pub items: Vec<BoxAST>
}

impl From<&mut ParseNode> for ArrayNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			items: node.map_mut(&parse)
		}
	}
}

impl AST for ArrayNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let mut items: Vec<Value> = vec![];
		for array_item in self.items.iter() {
			let item = array_item.resolve(scope)?;
			items.push(item.value);
		}
		Ok(ValueNode::from_value(Value::Array(items)))
	}
}