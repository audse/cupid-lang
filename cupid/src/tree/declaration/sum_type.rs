use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumTypeDeclaration {
	pub symbol: TypeHintNode,
	pub types: Vec<TypeHintNode>,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for Result<SumTypeDeclaration, Error> {
	fn from(node: &mut ParseNode) -> Self {
		let generics = if let Some(generics) = Result::<Option<GenericsNode>, Error>::from_parent(node)? {
			generics.0
		} else {
			vec![]
		};
		let i = if !generics.is_empty() { 1 } else { 0 };
		let name = &node.children[i].tokens[0].source;
    	Ok(SumTypeDeclaration {
			symbol: TypeHintNode::new(name.to_owned(), vec![TypeFlag::Sum], generics, node.children[0].tokens.to_owned()),
			types: node.filter_map_mut_result(&|child| if &*child.name == "sum_member" {
				Some(Result::<TypeHintNode, Error>::from(&mut child.children[0]))
			} else {
				None
			})?,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for SumTypeDeclaration {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let symbol = SymbolNode::from(&self.symbol);
		
		let types: Vec<TypeHintNode> = self.types.iter().cloned().collect();
		
		let type_value = TypeKind::Sum(SumType {
			types,
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