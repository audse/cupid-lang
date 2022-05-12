use crate::*;

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct GenericNode {
// 	pub identifier: Cow<'static, str>,
// 	pub type_value: Option<TypeHintNode>,
// 	pub meta: Meta<()>,
// }
// 
// impl From<&mut ParseNode> for GenericNode {
// 	fn from(node: &mut ParseNode) -> Self {
// 		let identifier = node.children[0].tokens[0].source.to_owned();
// 		let meta = Meta::with_tokens(node.tokens.to_owned());
// 		match node.children.len() {
// 			1 => Self { 
// 				identifier, 
// 				type_value: None, 
// 				meta
// 			},
// 			2 => Self {
// 				identifier,
// 				type_value: Some(TypeHintNode::from(&mut node.children[1])),
// 				meta
// 			},
// 			_ => panic!("too many children for generic node")
// 		}
// 	}
// }
// 
// impl AST for GenericNode {
// 	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
// 		let type_kind = self.resolve_to_generic(scope)?;	
// 		let type_value = if let Some(type_value) = type_kind.type_value {
// 			vec![*type_value]
// 		} else {
// 			vec![]
// 		};
// 		let type_id = TypeHintNode::new_symbol(type_kind.identifier, type_value);
// 		Ok(ValueNode {
// 			value: Value::TypeHintNode(type_id),
// 			type_hint: None,
// 			meta: Meta::<Flag>::from(&self.meta)
// 		})
// 	}
// }
// 
// impl GenericNode {
// 	fn resolve_to_generic(&self, scope: &mut LexicalScope) -> Result<GenericType, Error> {
// 		let type_value = if let Some(type_hint) = self.type_value {
// 			Some(Box::new(type_hint.resolve_to_type_kind(scope)?))
// 		} else {
// 			None
// 		};
// 		Ok(GenericType {
// 			identifier: self.identifier,
// 			type_value
// 		})
		// let type_symbol = SymbolNode(ValueNode {
		// 	value: Value::TypeHintNode(type_id),
		// 	type_kind: None,
		// 	meta: Meta::<Flag>::from(&self.meta)
		// });
		
		// let mut type_kind = self.identifier.0.type_kind.to_owned();
		// let type_value = self.type_value.as_ref();
		// if let Some(generic) = type_kind {
		// 	let mut type_value = SymbolNode::from(generic).resolve(scope)?;
		// 	if let Value::Type(TypeKind::Generic(ref mut g)) = &mut type_value.value {
		// 		
		// 	}
		// 	
		// 	generic.type_value = if let Some(type_value) = type_value {
		// 		let type_value = type_value.resolve_to_type_kind(scope)?;
		// 		Some(Box::new(type_value))
		// 	} else {
		// 		None
		// 	};
		// 	Ok(generic.to_owned())
		// } else {
		// 	panic!("expected a generic")
		// }
// 	}
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenericsNode(pub Vec<TypeHintNode>);

impl FromParent<&mut ParseNode> for Option<GenericsNode> {
	fn from_parent(node: &mut ParseNode) -> Self {
		let generics_node = node.children.iter_mut().find(|n| &*n.name == "generics");
		generics_node.map(GenericsNode::from)
	}
}

impl From<&mut ParseNode> for GenericsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.children.iter_mut().map(|g| TypeHintNode::generic(
			g.children[0].tokens[0].source.to_owned(),
			g.children[0].tokens.to_owned() // TODO args
		)).collect())
	}
}

impl AST for GenericsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	todo!()
	}
}

impl ResolveTo<Vec<GenericType>> for GenericsNode {
	fn resolve_to(&self, _scope: &mut LexicalScope) -> Result<Vec<GenericType>, Error> {
    	let mut generic_types: Vec<GenericType> = vec![];
		for generic in self.0.iter() {
			generic_types.push(GenericType {
				identifier: generic.identifier.to_owned(),
				type_value: if !generic.args.is_empty() {
					Some(generic.args[0].to_owned())
				} else {
					None
				}
			});
		}
		Ok(generic_types)
	}
}

impl GenericsNode {
	// pub fn resolve_to_generics(&self, scope: &mut LexicalScope) -> Result<Vec<GenericType>, Error> {
	// 	// scope.add(Context::Block);
	// 	let mut generics: Vec<GenericType> = vec![];
	// 	for generic in self.0.iter() {
	// 		let generic_type = generic.resolve_to_generic(scope)?;
	// 		// create_generic_symbol(&generic_type, &generic.identifier.0.meta, scope)?;
	// 		generics.push(generic_type);
	// 	}
	// 	// scope.pop();
	// 	Ok(generics)
	// }
	// pub fn find(&self, symbol: &SymbolNode) -> Option<&GenericNode> {
	// 	self.0.iter().find(|generic| Value::String(generic.identifier) == symbol.0.value)
	// }
	// pub fn create_symbols(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error>  {
	// 	let mut result = ValueNode::new_none();
	// 	for symbol in self.0.iter() {
	// 		let generic = symbol.resolve_to_generic(scope)?;
	// 		result = create_generic_symbol(&generic, &self.meta, scope)?;
	// 	}
	// 	Ok(result)
	// }
}