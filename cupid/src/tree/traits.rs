use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitNode {
	pub symbol: SymbolNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for TraitNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");
		let symbol_i = if let Some(_) = generics { 1 } else { 0 };
		Self {
			symbol: SymbolNode::from(&mut node.children[symbol_i]),
			functions: ImplementationNode::from(node),
		}
	}
}

impl AST for TraitNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let implementation = self.functions.resolve_to_implementation(scope)?;
		
		let symbol = SymbolNode::from((&self.symbol, &implementation.generics));
		
		let symbol_value = SymbolValue::Declaration { 
			type_hint: TypeKind::Type,
			value: ValueNode {
				type_kind: TypeKind::Type,
				value: Value::Implementation(implementation),
				meta: self.functions.2.to_owned(),
			},
			mutable: false,
		};
		
		scope.set_symbol(&symbol, symbol_value)
	}
}