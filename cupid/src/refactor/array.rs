use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArrayNode {
	pub items: Vec<BoxAST>
}

impl From<&mut ParseNode> for ArrayNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			items: node.map_mut(&|i| BoxAST::from(parse(i)))
		}
	}
}

impl AST for ArrayNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
    	let mut items: Vec<Value> = vec![];
		for array_item in self.items.iter() {
			let item = array_item.resolve(scope)?;
			items.push(item.value);
		}
		Ok(ValueNode::from_value(Value::Array(items)))
	}
}