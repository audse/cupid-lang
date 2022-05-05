use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UseBlockNode {
	pub type_kind: TypeHintNode,
	pub functions: Vec<DeclarationNode>,
	pub generics: Option<GenericsNode>,
}

impl From<&mut ParseNode> for UseBlockNode {
	fn from(node: &mut ParseNode) -> Self {
		let generics = GenericsNode::from_parent(node);
		
		let functions: Vec<DeclarationNode> = node.children
			.iter_mut()
			.filter_map(|n| if n.name.as_str() == "typed_declaration" {
				Some(DeclarationNode::from(n))
			} else {
				None
			}) 
			.collect();
			
		let type_kind = if generics.is_some() {
			&mut node.children[1]
		} else {
			&mut node.children[0]
		};
		let type_kind = TypeHintNode::from(type_kind);
		
		Self {
			type_kind,
			functions,
			generics,
		}
	}
}

impl AST for UseBlockNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut type_kind = self.type_kind.resolve_to_type_kind(scope)?;
		let mut functions = HashMap::new();
		for function in self.functions.iter() {
			let value = function.value.resolve(scope)?;
			// TODO change from `Value` to `ValueNode`
			functions.insert(function.symbol.0.value.to_owned(), value.value);
		}
		match type_kind.implement(functions) {
			Ok(_) => (),
			Err(_) => panic!(), // TODO
		};
		let symbol_value = RSymbolValue::Assignment { 
			value: ValueNode::from_value(Value::Type(type_kind)) 
		};
		scope.set_symbol(&self.type_kind.type_kind, &symbol_value)
	}
}