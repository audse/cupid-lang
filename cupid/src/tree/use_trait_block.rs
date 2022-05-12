use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseTraitBlockNode {
	pub trait_symbol: TypeHintNode,
	pub type_symbol: TypeHintNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for UseTraitBlockNode {
	fn from(node: &mut ParseNode) -> Self {
		let implementation = ImplementationNode::from(&mut node.to_owned());
		let generics = node.get_mut("generics");
		let (trait_i, type_kind_i) = if let Some(_) = generics {
			(1, 2)
		} else {
			(0, 1)
		};
		Self {
			trait_symbol: TypeHintNode {
				identifier: node.children[trait_i].tokens[0].source.to_owned(),
				args: if let Some(generics) = implementation.1 {
					generics.0.to_owned()
				} else {
					vec![]
				},
				meta: Meta::with_tokens(node.tokens.to_owned())
			},
			type_symbol: TypeHintNode::from(&mut node.children[type_kind_i]),
			functions: ImplementationNode::from(node),
		}
	}
}

impl AST for UseTraitBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let trait_symbol = SymbolNode::from(&self.trait_symbol);
		let type_symbol = SymbolNode::from(&self.type_symbol);
		
		let implementation = self.functions.resolve_to(scope)?;
		let mut trait_value = trait_symbol.resolve(scope)?;
		
		if let Value::Implementation(ref mut trait_value) = trait_value.value {
			trait_value.generics = implementation.generics;
			trait_value.implement(implementation.functions);
			
			let symbol_value = SymbolValue::Implementation { 
				trait_symbol: Some(self.trait_symbol.to_owned()),
				value: trait_value.to_owned()
			};
			scope.set_symbol(&type_symbol, symbol_value)
		} else {
			Err(trait_value.error_raw("expected a trait"))
		}
	}
}