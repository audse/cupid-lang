use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseTraitBlockNode {
	pub trait_name: SymbolNode,
	pub type_kind: TypeHintNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for UseTraitBlockNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");
		let (trait_i, type_kind_i) = if let Some(_) = generics {
			(1, 2)
		} else {
			(0, 1)
		};
		Self {
			trait_name: SymbolNode::from(&mut node.children[trait_i]),
			type_kind: TypeHintNode::from(&mut node.children[type_kind_i]),
			functions: ImplementationNode::from(node),
		}
	}
}

impl AST for UseTraitBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		scope.add(Context::Implementation);
		
		let type_symbol = self.type_kind.to_symbol(scope)?;
		
		let implementation = self.functions.resolve_to_implementation(scope)?;
		
		let trait_symbol = SymbolNode::from((&self.trait_name, &implementation.generics));
		let mut trait_value = trait_symbol.resolve(scope)?;
		
		if let Some(generics) = &self.functions.1 {
			generics.create_symbols(scope)?;
		}
		
		if let Value::Implementation(ref mut trait_value) = trait_value.value {
			trait_value.generics = implementation.generics;
			trait_value.implement(implementation.functions);
			let symbol_value = SymbolValue::Implementation { 
				trait_symbol: Some(self.trait_name.to_owned()),
				value: trait_value.to_owned()
			};
			let value = scope.set_symbol(&type_symbol, symbol_value);
			scope.pop();
			value
		} else {
			Err(trait_value.error_raw("expected a trait"))
		}
	}
}