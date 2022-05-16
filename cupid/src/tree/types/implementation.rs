use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationNode {
	pub functions: Vec<DeclarationNode>, 
	pub type_generics: Option<GenericsNode>, 
	pub trait_generics: Option<GenericsNode>, 
	pub meta: Meta<Flag>
}

impl From<&mut ParseNode> for ImplementationNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut generics = Vec::<GenericsNode>::from_parent(node);
		generics.reverse();
		let type_generics = if node.child_is(0, "generics") {
			generics.pop()
		} else {
			None
		};
		let trait_generics = generics.pop();
		Self {
			functions: node.children
				.iter_mut()
				.filter_map(|n| if &*n.name == "typed_declaration" {
					Some(DeclarationNode::from(n))
				} else {
					None
				}) 
				.collect(),
			type_generics,
			trait_generics,
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for ImplementationNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let implementation: Implementation = self.resolve_to(scope)?;
		Ok(ValueNode {
			type_hint: None,
			value: Value::Implementation(implementation),
			meta: self.meta.to_owned(),
		})
	}
}

impl ImplementationNode {
	pub fn make(&self, generics: &[GenericType], scope: &mut LexicalScope) -> Result<Implementation, Error> {
		// creates hashmap of functions/symbols and applies generic types wherever needed
		let mut implementation = Implementation::default();
		implementation.generics = generics.to_owned();
		for function in self.functions.iter() {
			let value = function.value.resolve(scope)?;			
			implementation.functions.insert(function.symbol.0.to_owned(), value);
		}
		Ok(implementation)
	}
}

impl ResolveTo<Implementation> for ImplementationNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<Implementation, Error> {
		let generics: Vec<GenericType> = if let Some(generics) = &self.type_generics {
			generics.resolve_to(scope)?
		} else { 
			vec![] 
		};
		self.make(&generics, scope)
	}
}