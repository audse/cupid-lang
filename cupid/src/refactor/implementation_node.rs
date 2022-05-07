use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImplementationNode(pub Vec<DeclarationNode>, pub Meta<Flag>);

impl From<&mut ParseNode> for ImplementationNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.children
				.iter_mut()
				.filter_map(|n| if n.name.as_str() == "typed_declaration" {
					Some(DeclarationNode::from(n))
				} else {
					None
				}) 
				.collect(),
			Meta::with_tokens(node.tokens.to_owned())
		)
	}
}

impl AST for ImplementationNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut functions = Implementation::new();
		for function in self.0.iter() {
			let value = function.value.resolve(scope)?;
			functions.functions.insert(function.symbol.0.to_owned(), value);
		}
		Ok(ValueNode {
			type_kind: TypeKind::Type,
			value: Value::Implementation(functions),
			meta: self.1.to_owned(),
		})
	}
}

impl ImplementationNode {
	pub fn resolve_to_implementation(&self, scope: &mut RLexicalScope) -> Result<Implementation, Error> {
		match self.resolve(scope) {
			Ok(val) => if let Value::Implementation(val) = val.value {
				Ok(val)
			} else {
				Err(val.error_raw("expected implementation"))
			},
			Err(e) => Err(e)
		}
	}
}