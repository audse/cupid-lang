use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructTypeDeclaration {
	pub symbol: SymbolNode,
	pub members: Vec<(TypeHintNode, SymbolNode)>,
	pub generics: Option<GenericsNode>,
	pub meta: Meta<()>,
}

impl From<&mut ParseNode> for StructTypeDeclaration {
	fn from(node: &mut ParseNode) -> Self {
		let generics = Option::<GenericsNode>::from_parent(node);
		let i = if generics.is_some() { 1 } else { 0 };
		Self {
			generics,
			symbol: SymbolNode::from(&mut node.children[i]),
			members: node.filter_map_mut(&|child| if &*child.name == "struct_member" {
				Some((
					TypeHintNode::from(&mut child.children[0]), 
					SymbolNode::from(&mut child.children[1])
				))
			} else {
				None
			}),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for StructTypeDeclaration {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let generics: Vec<GenericType> = if let Some(generics) = &self.generics {
			generics.resolve_to_generics(scope)?
		} else {
			vec![]
		};
		let symbol = SymbolNode::from((&self.symbol, &generics));
		
		let mut members = vec![];
		for (type_value, member_name) in self.members.iter() {
			let type_value = type_value.resolve_to_type_kind(scope)?;
			members.push((member_name.0.to_owned(), type_value));
		}
		
		let type_value = TypeKind::Struct(StructType {
			members,
			implementation: Implementation::default()
		});
		
		let declare = SymbolValue::Declaration { 
			type_hint: TypeKind::Type, 
			mutable: false, 
			value: ValueNode::from((Value::Type(type_value), &symbol.0.meta)),
		};
		scope.set_symbol(&symbol, declare)
	}
}