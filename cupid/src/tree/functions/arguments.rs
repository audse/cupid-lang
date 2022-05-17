use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentsNode(pub Vec<BoxAST>);

impl From<&mut ParseNode> for ArgumentsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.map_mut(&parse))
	}
}

impl AST for ArgumentsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		unreachable!("cannot resolve arguments as a whole")
	}
}

impl ResolveTo<Vec<ValueNode>> for ArgumentsNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<Vec<ValueNode>, Error> {
		let mut values = vec![];
		for arg in self.0.iter() {
			let value = arg.resolve(scope)?;
			values.push(value);
		}
		Ok(values)
	}
}

impl ArgumentsNode {
	pub fn empty(&self) -> bool { self.0.is_empty() }
}