use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseBlockNode {
	pub type_kind: TypeHintNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for UseBlockNode {
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

impl AST for UseBlockNode {
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