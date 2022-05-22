use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhileLoopNode {
	pub condition: BoxAST,
	pub body: BlockNode
}

impl FromParse for Result<WhileLoopNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
    	Ok(WhileLoopNode {
			condition: parse(&mut node.children[0])?,
			body: Result::<BlockNode, Error>::from_parse(&mut node.children[1])?
		})
	}
}

impl AST for WhileLoopNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = ValueNode::new_none();
    	loop {
			scope.add(Context::Loop);
			let condition = self.condition.resolve(scope)?;
			if let Value::Boolean(c) = condition.value {
				if c {
					value = self.body.resolve(scope)?;
				} else {
					scope.pop();
					break;
				}
			} else {
				return Err(condition.error(format!("expected a boolean condition, found {condition}"), scope))
			}
			scope.pop();
		}
		Ok(value)
	}
}

impl Display for WhileLoopNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}