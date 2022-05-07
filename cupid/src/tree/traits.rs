use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitNode {
	pub symbol: SymbolNode,
	pub functions: ImplementationNode,
	pub generics: Option<GenericsNode>,
}

impl From<&mut ParseNode> for TraitNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");
		let (generics, symbol_i) = if generics.is_some() {
			(Some(GenericsNode::from(generics.unwrap())), 1)
		} else {
			(None, 0)
		};
		Self {
			symbol: SymbolNode::from(&mut node.children[symbol_i]),
			functions: ImplementationNode::from(node),
			generics,
		}
	}
}

impl AST for TraitNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut trait_value = Implementation::new();
		for function in self.functions.0.iter() {
			let value = function.value.resolve(scope)?;
			trait_value.functions.insert(function.symbol.0.to_owned(), value);
		}
		let symbol_value = SymbolValue::Declaration { 
			type_hint: TypeKind::Type,
			value: ValueNode {
				type_kind: TypeKind::Type,
				value: Value::Implementation(trait_value),
				meta: Meta::new(vec![], None, vec![]),
			},
			mutable: false,
		};
		scope.set_symbol(&self.symbol, &symbol_value)
	}
}