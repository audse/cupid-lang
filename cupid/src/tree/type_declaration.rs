use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AliasTypeDeclaration<'src> {
	pub symbol: SymbolNode<'src>,
	pub type_kind: TypeHintNode<'src>,
	pub meta: Meta<'src, ()>,
}

impl<'src> From<&mut ParseNode<'src>> for AliasTypeDeclaration<'src> {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			symbol: SymbolNode::from(&mut node.children[0]),
			type_kind: TypeHintNode::from(&mut node.children[1]),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl<'src> AST for AliasTypeDeclaration<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let type_value = self.type_kind.resolve(scope)?;
		let declare = SymbolValue::Declaration { 
			type_hint: TypeKind::Type, 
			mutable: false, 
			value: type_value,
		};
		scope.set_symbol(&self.symbol, declare)
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructTypeDeclaration;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumTypeDeclaration;
