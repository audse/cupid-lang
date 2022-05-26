use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseTraitBlockNode {
	pub trait_symbol: TypeHintNode,
	pub type_symbol: TypeHintNode,
	pub functions: ImplementationNode,
}

impl FromParse for Result<UseTraitBlockNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let implementation = Result::<ImplementationNode, Error>::from_parse(&mut node.to_owned())?;
		let (trait_i, type_kind_i) = match (&implementation.type_generics, &implementation.trait_generics) {
			(Some(_), Some(_)) => (1, 3),
			(Some(_), None) => (1, 2),
			(None, Some(_)) => (0, 2),
			(None, None) => (0, 1)
		};
		Ok(UseTraitBlockNode {
			trait_symbol: TypeHintNode {
				identifier: node.children[trait_i].tokens[0].source.to_owned(),
				args: if let Some(generics) = implementation.trait_generics {
					generics.0.to_owned()
				} else {
					vec![]
				},
				meta: Meta::with_tokens(node.tokens.to_owned())
			},
			type_symbol: Result::<TypeHintNode, Error>::from_parse(&mut node.children[type_kind_i])?,
			functions: Result::<ImplementationNode, Error>::from_parse(node)?,
		})
	}
}

impl AST for UseTraitBlockNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut trait_symbol = SymbolNode::from(&self.trait_symbol);
		let mut type_symbol = SymbolNode::from(&self.type_symbol);
		trait_symbol.0.meta.set_token_store(scope);
		type_symbol.0.meta.set_token_store(scope);
		
		let implementation = self.functions.resolve_to(scope)?;
		
		let mut trait_generics = if let Some(trait_generics) = &self.functions.trait_generics {
			trait_generics.resolve_to(scope)?
		} else {
			vec![]
		};
		let mut type_generics = if let Some(type_generics) = &self.functions.type_generics {
			type_generics.resolve_to(scope)?
		} else {
			vec![]
		};
		trait_generics.append(&mut type_generics);
		
		let mut trait_value = trait_symbol.resolve(scope)?;
		trait_value.meta.set_token_store(scope);
		
		if let Value::Implementation(ref mut trait_value) = trait_value.value {
			trait_value.generics = trait_generics;
			trait_value.implement(implementation.functions);
			
			let symbol_value = SymbolValue::Implementation { 
				trait_symbol: Some(self.trait_symbol.to_owned()),
				value: trait_value.to_owned()
			};
			scope.set_symbol(&type_symbol, symbol_value)
		} else {
			Err(trait_value.error(format!("expected a trait, found {trait_value}"), scope))
		}
	}
}

impl Display for UseTraitBlockNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "use {} with {} {}", self.trait_symbol, self.type_symbol, self.functions)
	}
}