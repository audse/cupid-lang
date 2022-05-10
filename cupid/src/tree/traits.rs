use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitNode<'src> {
	pub symbol: SymbolNode<'src>,
	pub functions: ImplementationNode<'src>,
}

impl<'src> From<&mut ParseNode<'src>> for TraitNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");
		let symbol_i = if let Some(_) = generics { 1 } else { 0 };
		Self {
			symbol: SymbolNode::from(&mut node.children[symbol_i]),
			functions: ImplementationNode::from(node),
		}
	}
}

impl<'src> AST for TraitNode<'src> {
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