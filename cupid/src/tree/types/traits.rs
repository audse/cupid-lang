use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitNode {
	pub symbol: TypeHintNode,
	pub functions: ImplementationNode,
}

impl From<&mut ParseNode> for Result<TraitNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		let generics = if let Some(generics) = Result::<Option<GenericsNode>, Error>::from_parent(node)? {
			generics.0
		} else {
			vec![]
		};
		let i = if !generics.is_empty() { 1 } else { 0 };
		let name = node.children[i].tokens[0].source.to_owned();
		Ok(TraitNode {
			symbol: TypeHintNode::new(name, vec![TypeFlag::Trait], generics, node.children[0].tokens.to_owned()),
			functions: Result::<ImplementationNode, Error>::from(node)?,
		})
	}
}

impl AST for TraitNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let symbol = SymbolNode::from(&self.symbol);
		let mut type_generics = if let Some(generics) = &self.functions.type_generics {
			generics.resolve_to(scope)?
		} else {
			vec![]
		};
		let mut trait_generics = if let Some(generics) = &self.functions.trait_generics {
			generics.resolve_to(scope)?
		} else {
			vec![]
		};
		type_generics.append(&mut trait_generics);
		
		scope.add(Context::Implementation);
		for generic in type_generics {
			create_generic_symbol(&generic, &self.functions.meta, scope)?;
		}
		let implementation = self.functions.resolve_to(scope)?;
		scope.pop();
		
		let symbol_value = SymbolValue::Declaration { 
			type_hint: None,
			value: ValueNode {
				type_hint: None,
				value: Value::Implementation(implementation),
				meta: self.functions.meta.to_owned(),
			},
			mutable: false,
		};
		
		scope.set_symbol(&symbol, symbol_value)
	}
}