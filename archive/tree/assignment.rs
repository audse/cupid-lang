use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssignmentNode {
	pub symbol: SymbolNode,
	pub value: BoxAST,
	pub meta: Meta<()>
}

impl FromParse for Result<AssignmentNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
    	Ok(AssignmentNode {
			symbol: Result::<SymbolNode, Error>::from_parse(&mut node.children[0])?,
			value: parse(&mut node.children[1])?,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for AssignmentNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		scope.set_symbol(&self.symbol, SymbolValue::Assignment { value })
	}
}

impl Display for AssignmentNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{} = {}", self.symbol, self.value)
	}
}