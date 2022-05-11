use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenericNode {
	pub identifier: SymbolNode,
	pub type_value: Option<TypeHintNode>,
}

impl From<&mut ParseNode> for GenericNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut identifier = SymbolNode::from(&mut node.children[0]);
		identifier.0.type_kind = TypeKind::new_generic(identifier.get_identifier_string());
		match node.children.len() {
			1 => Self { identifier, type_value: None },
			2 => Self {
				identifier,
				type_value: Some(TypeHintNode::from(&mut node.children[1]))
			},
			_ => panic!("too many children for generic node")
		}
	}
}

impl AST for GenericNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let type_kind = self.resolve_to_generic(scope)?;	
		let type_id = TypeId::from(TypeKind::Generic(type_kind));
		Ok(ValueNode {
			value: Value::from(Value::TypeIdentifier(type_id)),
			type_kind: TypeKind::Type,
			meta: self.identifier.0.meta.to_owned()
		})
	}
}

impl GenericNode {
	fn resolve_to_generic(&self, scope: &mut LexicalScope) -> Result<GenericType, Error> {
		let mut type_kind = self.identifier.0.type_kind.to_owned();
		let type_value = self.type_value.as_ref();
		if let TypeKind::Generic(ref mut generic) = &mut type_kind {
			generic.type_value = if let Some(type_value) = type_value {
				let type_value = type_value.resolve_to_type_kind(scope)?;
				Some(Box::new(type_value))
			} else {
				None
			};
			Ok(generic.to_owned())
		} else {
			panic!("expected a generic")
		}
	}
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenericsNode(pub Vec<GenericNode>);

impl FromParent<&mut ParseNode> for Option<GenericsNode> {
	fn from_parent(node: &mut ParseNode) -> Self {
		let generics_node = node.children.iter_mut().find(|n| &*n.name == "generics");
		generics_node.map(GenericsNode::from)
	}
}

impl From<&mut ParseNode> for GenericsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.children.iter_mut().map(GenericNode::from).collect())
	}
}

impl AST for GenericsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	todo!()
	}
}

impl GenericsNode {
	pub fn resolve_to_generics(&self, scope: &mut LexicalScope) -> Result<Vec<GenericType>, Error> {
		scope.add(Context::Block);
		let mut generics: Vec<GenericType> = vec![];
		for generic in self.0.iter() {
			let generic_type = generic.resolve_to_generic(scope)?;
			create_generic_symbol(&generic_type, &generic.identifier.0.meta, scope)?;
			generics.push(generic_type);
		}
		scope.pop();
		Ok(generics)
	}
	pub fn find(&self, symbol: &SymbolNode) -> Option<&GenericNode> {
		self.0.iter().find(|generic| generic.identifier.0.value == symbol.0.value)
	}
	pub fn create_symbols(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error>  {
		let mut result = ValueNode::new_none();
		for symbol in self.0.iter() {
			let generic = symbol.resolve_to_generic(scope)?;
			result = create_generic_symbol(&generic, &symbol.identifier.0.meta, scope)?;
		}
		Ok(result)
	}
}