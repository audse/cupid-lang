use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNode {
	pub expressions: Vec<BoxAST>,
}

impl From<&mut ParseNode> for Result<BlockNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		Ok(BlockNode {
			expressions: node.map_mut_result(&parse)?,
		})
	}
}

impl AST for BlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut values: Vec<ValueNode> = vec![
			ValueNode::new_none()
		];
		for exp in self.expressions.iter() {
			let value = exp.resolve(scope)?;
			values.push(value);
		}
		Ok(values.pop().unwrap())
	}
}