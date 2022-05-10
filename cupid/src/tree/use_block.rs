use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseBlockNode<'src> {
	pub type_kind: TypeHintNode<'src>,
	pub functions: ImplementationNode<'src>,
}

impl<'src> From<&mut ParseNode<'src>> for UseBlockNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");	
		let type_kind = if generics.is_some() {
			&mut node.children[1]
		} else {
			&mut node.children[0]
		};
		let type_kind = TypeHintNode::from(type_kind);
		Self {
			type_kind,
			functions: ImplementationNode::from(node),
		}
	}
}

impl<'src> AST for UseBlockNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let implementation = self.functions.resolve_to_implementation(scope)?;
		let type_symbol = self.type_kind.to_symbol(scope)?;
		let symbol_value = SymbolValue::Implementation { 
			trait_symbol: None,
			value: implementation,
		};
		scope.set_symbol(&type_symbol, symbol_value)
	}
}