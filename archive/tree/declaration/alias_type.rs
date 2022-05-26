use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasTypeDeclaration {
	pub symbol: SymbolNode,
	pub type_kind: TypeHintNode,
	pub meta: Meta<()>,
}

impl FromParse for Result<AliasTypeDeclaration, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		Ok(AliasTypeDeclaration {
			symbol: Result::<SymbolNode, Error>::from_parse(&mut node.children[0])?,
			type_kind: Result::<TypeHintNode, Error>::from_parse(&mut node.children[1])?,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for AliasTypeDeclaration {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let type_value = self.type_kind.resolve(scope)?;
		let declare = SymbolValue::Declaration { 
			type_hint: None, 
			mutable: false, 
			value: type_value,
		};
		scope.set_symbol(&self.symbol, declare)
	}
}

impl Display for AliasTypeDeclaration {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}