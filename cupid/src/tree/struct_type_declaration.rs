use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructTypeDeclaration {
	pub symbol: TypeHintNode,
	pub members: Vec<(TypeHintNode, SymbolNode)>,
	pub meta: Meta<()>,
}

impl From<&mut ParseNode> for StructTypeDeclaration {
	fn from(node: &mut ParseNode) -> Self {
		let generics = if let Some(generics) = Option::<GenericsNode>::from_parent(node) {
			generics.0
		} else {
			vec![]
		};
		let i = if !generics.is_empty() { 1 } else { 0 };
		let name = node.children[i].tokens[0].source.to_owned();
		Self {
			symbol: TypeHintNode::new(name, TypeFlag::Struct, generics, node.children[0].tokens.to_owned()),
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
		let symbol = SymbolNode::from(&self.symbol);
		
		let mut members: Vec<(ValueNode, TypeHintNode)> = vec![];
		for (type_value, member_name) in self.members.iter() {
			members.push((member_name.0.to_owned(), type_value.to_owned()));
		}
		
		let type_value = TypeKind::Struct(StructType {
			members,
			implementation: Implementation::default()
		});
		
		let declare = SymbolValue::Declaration { 
			type_hint: None, 
			mutable: false, 
			value: ValueNode::from((Value::Type(type_value), &symbol.0.meta)),
		};
		scope.set_symbol(&symbol, declare)
	}
}