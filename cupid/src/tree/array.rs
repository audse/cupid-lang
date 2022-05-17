use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayNode {
	pub items: Vec<BoxAST>,
	pub meta: Meta<Flag>,
}

impl From<&mut ParseNode> for ArrayNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			items: node.map_mut(&parse),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for ArrayNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let mut items: Vec<ValueNode> = vec![];
		for array_item in self.items.iter() {
			let item = array_item.resolve(scope)?;
			items.push(item);
		}
		Ok(ValueNode::from((Value::Array(items), &self.meta)))
	}
}