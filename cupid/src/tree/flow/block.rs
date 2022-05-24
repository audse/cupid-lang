use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNode {
	pub expressions: Vec<BoxAST>,
}

impl FromParse for Result<BlockNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		Ok(BlockNode {
			expressions: node.map(&parse)?,
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

impl Display for BlockNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let expressions: Vec<String> = self.expressions.iter().map(|e| e.to_string()).collect();
		write!(f, "{{ {} }}", expressions.join("\n"))
	}
}