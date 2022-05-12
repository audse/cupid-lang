use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitNode {
	pub symbol: TypeHintNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for TraitNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = if let Some(generics) = Option::<GenericsNode>::from_parent(node) {
			generics.0
		} else {
			vec![]
		};
		let i = if !generics.is_empty() { 1 } else { 0 };
		let name = node.children[i].tokens[0].source.to_owned();
		Self {
			symbol: TypeHintNode::new(name, TypeFlag::Trait, generics, node.children[0].tokens.to_owned()),
			functions: ImplementationNode::from(node),
		}
	}
}

impl AST for TraitNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let symbol = SymbolNode::from(&self.symbol);
		
		let generics = if let Some(generics) = &self.functions.1 {
			generics.resolve_to(scope)?
		} else {
			vec![]
		};
		scope.add(Context::Implementation);
		for generic in generics {
			create_generic_symbol(&generic, &self.functions.2, scope)?;
		}
		let implementation = self.functions.resolve_to(scope)?;
		scope.pop();
		
		let symbol_value = SymbolValue::Declaration { 
			type_hint: None,
			value: ValueNode {
				type_hint: None,
				value: Value::Implementation(implementation),
				meta: self.functions.2.to_owned(),
			},
			mutable: false,
		};
		
		scope.set_symbol(&symbol, symbol_value)
	}
}