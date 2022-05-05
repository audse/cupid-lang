use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockNode {
	pub expressions: Vec<BoxAST>,
}

impl From<&mut ParseNode> for BlockNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			expressions: node.children.iter_mut().map(|exp| BoxAST::from(parse(exp))).collect(),
		}
	}
}

impl AST for BlockNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut values: Vec<ValueNode> = vec![
			ValueNode::from_value(Value::None)
		];
		for exp in self.expressions.iter() {
			let value = exp.resolve(scope)?;
			values.push(value);
		}
		Ok(values.pop().unwrap())
	}
}