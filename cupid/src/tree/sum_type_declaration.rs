use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumTypeDeclaration {
	pub symbol: SymbolNode,
	pub types: Vec<TypeHintNode>,
	pub generics: Option<GenericsNode>,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for SumTypeDeclaration {
	fn from(node: &mut ParseNode) -> Self {
		let generics = Option::<GenericsNode>::from_parent(node);
		let i = if generics.is_some() { 1 } else { 0 };
    	Self {
			generics,
			symbol: SymbolNode::from(&mut node.children[i]),
			types: node.filter_map_mut(&|child| if &*child.name == "sum_member" {
				Some(TypeHintNode::from(&mut child.children[0]))
			} else {
				None
			}),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for SumTypeDeclaration {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let generics: Vec<GenericType> = if let Some(generics) = &self.generics {
			generics.resolve_to_generics(scope)?
		} else {
			vec![]
		};
		let symbol = SymbolNode::from((&self.symbol, &generics));
		
		let mut types = vec![];
		for type_value in self.types.iter() {
			let type_value = type_value.resolve_to_type_kind(scope)?;
			types.push(type_value);
		}
		
		let type_value = TypeKind::Sum(SumType {
			types,
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