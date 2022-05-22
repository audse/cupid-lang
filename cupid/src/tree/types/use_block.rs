use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseBlockNode {
	pub type_kind: TypeHintNode,
	pub functions: ImplementationNode,
}

impl FromParse for Result<UseBlockNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");	
		let type_kind = if generics.is_some() {
			&mut node.children[1]
		} else {
			&mut node.children[0]
		};
		let type_kind = Result::<TypeHintNode, Error>::from_parse(type_kind)?;
		let implementation =  Result::<ImplementationNode, Error>::from_parse(node)?;
		Ok(UseBlockNode {
			type_kind,
			functions: implementation
		})
	}
}

impl AST for UseBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let implementation = self.functions.resolve_to(scope)?;
		let type_symbol = SymbolNode::from(&self.type_kind);
		let symbol_value = SymbolValue::Implementation { 
			trait_symbol: None,
			value: implementation,
		};
		scope.set_symbol(&type_symbol, symbol_value)
	}
}

impl Display for UseBlockNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "use {} {}", self.type_kind, self.functions)
	}
}