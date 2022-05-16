use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AliasTypeDeclaration {
	pub symbol: SymbolNode,
	pub type_kind: TypeHintNode,
	pub meta: Meta<()>,
}

impl From<&mut ParseNode> for AliasTypeDeclaration {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			symbol: SymbolNode::from(&mut node.children[0]),
			type_kind: TypeHintNode::from(&mut node.children[1]),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
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
