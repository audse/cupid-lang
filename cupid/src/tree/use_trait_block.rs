use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseTraitBlockNode {
	pub trait_name: SymbolNode,
	pub type_kind: TypeHintNode,
	pub functions: ImplementationNode,
	pub generics: Option<GenericsNode>
}

impl From<&mut ParseNode> for UseTraitBlockNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = node.get_mut("generics");
		let (generics, trait_i, type_kind_i) = if generics.is_some() {
			(Some(GenericsNode::from(generics.unwrap())), 1, 2)
		} else {
			(None, 0, 1)
		};
		Self {
			trait_name: SymbolNode::from(&mut node.children[trait_i]),
			type_kind: TypeHintNode::from(&mut node.children[type_kind_i]),
			generics,
			functions: ImplementationNode::from(node),
		}
	}
}

impl AST for UseTraitBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut trait_value = self.trait_name.resolve(scope)?;
		let mut type_kind = self.type_kind.resolve_to_type_kind(scope)?;
		let functions = self.functions.resolve_to_implementation(scope)?;
		
		if let Value::Implementation(ref mut trait_value) = trait_value.value {
			_ = trait_value.implement(functions.functions);
			match type_kind.implement_trait(self.trait_name.to_owned(), trait_value.functions.to_owned()) {
				Ok(_) => (),
				Err(_) => panic!(), // TODO
			};
			let symbol_value = SymbolValue::Assignment { 
				value: ValueNode::from_value(Value::Type(type_kind)) 
			};
			scope.set_symbol(&self.type_kind.type_kind, &symbol_value)
		} else {
			Err(trait_value.error_raw("expected a trait"))
		}
	}
}