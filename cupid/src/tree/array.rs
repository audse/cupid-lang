use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArrayNode<'src> {
	pub items: Vec<BoxAST>,
	pub meta: Meta<'src, Flag>,
}

impl<'src> From<&mut ParseNode<'src>> for ArrayNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			items: node.map_mut(&parse),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl<'src> AST for ArrayNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let mut items: Vec<ValueNode> = vec![];
		for array_item in self.items.iter() {
			let item = array_item.resolve(scope)?;
			items.push(item);
		}
		Ok(ValueNode::from((Value::Array(items), &self.meta)))
	}
}