use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImplementationNode(pub Vec<DeclarationNode>, pub Option<GenericsNode>, pub Meta<Flag>);

impl From<&mut ParseNode> for ImplementationNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(
			node.children
				.iter_mut()
				.filter_map(|n| if n.name.as_str() == "typed_declaration" {
					Some(DeclarationNode::from(n))
				} else {
					None
				}) 
				.collect(),
			Option::<GenericsNode>::from_parent(node),
			Meta::with_tokens(node.tokens.to_owned())
		)
	}
}

impl AST for ImplementationNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let generics = if let Some(generics) = &self.1 {
			generics.resolve_to_generics(scope)?
		} else { 
			vec![] 
		};
		let implementation = self.resolve_with_generics(&generics, scope)?;
		Ok(ValueNode {
			type_kind: TypeKind::Type,
			value: Value::Implementation(implementation),
			meta: self.2.to_owned(),
		})
	}
}

impl ImplementationNode {
	pub fn resolve_to_implementation(&self, scope: &mut LexicalScope) -> Result<Implementation, Error> {
		match self.resolve(scope) {
			Ok(val) => if let Value::Implementation(val) = val.value {
				Ok(val)
			} else {
				Err(val.error_raw("expected implementation"))
			},
			Err(e) => Err(e)
		}
	}
	pub fn resolve_with_generics(&self, generics: &[GenericType], scope: &mut LexicalScope) -> Result<Implementation, Error> {
		// creates hashmap of functions/symbols and applies generic types wherever needed
		let mut implementation = Implementation::default();
		implementation.generics = generics.to_owned();
		for function in self.0.iter() {
			let value = function.value.resolve(scope)?;			
			implementation.functions.insert(function.symbol.0.to_owned(), value);
		}
		Ok(implementation)
	}
}