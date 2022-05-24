use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayNode {
	pub items: Vec<BoxAST>,
	pub meta: Meta<Flag>,
}

impl FromParse for Result<ArrayNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		Ok(ArrayNode {
			items: node.map(&parse)?,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for ArrayNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let mut items: Vec<ValueNode> = vec![];
		for array_item in self.items.iter() {
			let item = array_item.resolve(scope)?;
			items.push(item);
		}
		let mut meta = self.meta.to_owned();
		meta.set_token_store(scope);
		
		Ok(ValueNode::from((Value::Array(items), meta)))
	}
}

impl Display for ArrayNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let items: Vec<String> = self.items.iter().map(|i| i.to_string()).collect();
		write!(f, "[{}]", items.join(", "))
	}
}