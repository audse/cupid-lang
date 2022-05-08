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
		let mut type_kind = self.type_kind.resolve_to_type_kind(scope)?;
		let implementation = self.functions.resolve_to_implementation(scope)?;
		type_kind.implement(implementation.functions);
		let symbol_value = SymbolValue::Assignment { 
			value: ValueNode::from_value(Value::Type(type_kind)) 
		};
		scope.set_symbol(&self.type_kind.type_kind, &symbol_value)
	}
}