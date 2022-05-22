use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationNode {
	pub functions: Vec<DeclarationNode>, 
	pub type_generics: Option<GenericsNode>, 
	pub trait_generics: Option<GenericsNode>, 
	pub meta: Meta<Flag>
}

impl FromParse for Result<ImplementationNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let mut generics = Result::<Vec<GenericsNode>, Error>::from_parent(node)?;
		generics.reverse();
		let type_generics = if node.child_is(0, "generics") {
			generics.pop()
		} else {
			None
		};
		let trait_generics = generics.pop();
		Ok(ImplementationNode {
			functions: node
				.filter_map_mut_result(&|n| if &*n.name == "typed_declaration" {
					Some(Result::<DeclarationNode, Error>::from_parse(n))
				} else {
					None
				})?,
			type_generics,
			trait_generics,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for ImplementationNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let implementation: Implementation = self.resolve_to(scope)?;
		let mut value = ValueNode {
			type_hint: None,
			value: Value::Implementation(implementation),
			meta: self.meta.to_owned(),
		};
		value.meta.set_token_store(scope);
		Ok(value)
	}
}

impl Display for ImplementationNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let functions: Vec<String> = self.functions.iter().map(|f| f.to_string()).collect();
		write!(f, 
			"[[{}][{}] {}]", 
			unwrap_or_string(&self.type_generics), 
			unwrap_or_string(&self.trait_generics),
			functions.join(", ")
		)
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