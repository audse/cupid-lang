use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfBlockNode {
	pub condition: Option<BoxAST>,
	pub body: BoxAST,
	pub else_if_blocks: Vec<Self>,
	pub else_block: Option<Box<Self>>
}

impl FromParse for Result<IfBlockNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let else_block: Option<Result<IfBlockNode, Error>> = node
			.get_mut("else_block")
			.map(|c| Self::from_parse(c));
		let else_block = match else_block {
			Some(Err(err)) => return Err(err),
			Some(Ok(block)) => Some(Box::new(block)),
			None => None
		};
		let (condition, body) = if node.children.len() > 1 {
			(Some(parse(&mut node.children[0])?), parse(&mut node.children[1])?)
		} else {
			(None, parse(&mut node.children[0])?)
		};
		Ok(IfBlockNode {
			condition,
			body,
			else_if_blocks: node.filter_map(&|child: &mut ParseNode| {
				if &*child.name == "else_if_block" {
					Some(Self::from_parse(child))
				} else {
					None
				}
			})?,
			else_block
		})
	}
}

impl AST for IfBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		if let Some(value) = self.resolve_to(scope)? {
			Ok(value)
		} else {
			for else_if_block in &self.else_if_blocks {
				if let Some(value) = else_if_block.resolve_to(scope)? {
					return Ok(value);
				}
			}
			if let Some(else_block) = &self.else_block {
				if let Some(value) = else_block.resolve_to(scope)? {
					return Ok(value)
				}
			}
			Ok(ValueNode::new_none())
		}
	}
}

impl ResolveTo<Option<ValueNode>> for IfBlockNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<Option<ValueNode>, Error> {
		if let Some(condition) = &self.condition {
			let condition = condition.resolve(scope)?;
			if let Value::Boolean(b) = &condition.value {
				if *b {
					let body = self.body.resolve(scope)?;
					Ok(Some(body))
				} else {
					Ok(None)
				}
			} else {
				Err(condition.error(format!("type mismatch: expected a boolean condition, not {condition}"), scope))
			}
		} else {
			Ok(Some(self.body.resolve(scope)?))
		}
	}
}

impl Display for IfBlockNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}