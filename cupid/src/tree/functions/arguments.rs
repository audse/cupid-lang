use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentsNode(pub Vec<BoxAST>);

impl FromParse for Result<ArgumentsNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		Ok(ArgumentsNode(node.map_mut_result(&parse)?))
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
	pub fn resolve_as_method(&self, scope: &mut LexicalScope) -> Result<(ValueNode, ArgumentsNode), Error> {
		let object = self.0.first().unwrap().resolve(scope)?;
		let args: Vec<BoxAST> = self.0.iter().skip(1).cloned().collect();
		Ok((object, ArgumentsNode(args)))
	}
	pub fn resolve_nth(&self, i: usize, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		self.0[i].resolve(scope)
	}
}

impl Display for ArgumentsNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let args: Vec<String> = self.0
			.iter()
			.map(|a| format!("{a}"))
			.collect();
    	write!(f, "{}", args.join(", "))
	}
}