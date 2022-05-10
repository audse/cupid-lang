use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode(pub ValueNode);

impl From<&mut ParseNode> for SymbolNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut value_node = ValueNode::from(node);
		// symbols do not have their own type
		value_node.type_kind = TypeKind::Placeholder;
    	Self(value_node)
	}
}

impl From<String> for SymbolNode {
    fn from(string: String) -> Self {
		let mut value_node = ValueNode::from(string);
		value_node.type_kind = TypeKind::Placeholder;
		Self(value_node)
	}
}

impl From<(String, &Meta<Flag>)> for SymbolNode {
	fn from(value: (String, &Meta<Flag>)) -> Self {
		Self(ValueNode::from((Value::String(value.0), value.1)))
	}
}

impl From<(&Self, &Vec<GenericType>)> for SymbolNode {
	fn from(symbol: (&Self, &Vec<GenericType>)) -> Self {
		let mut new_symbol = symbol.0.to_owned();
		new_symbol.0.value = Value::TypeIdentifier(TypeId(
			Box::new(symbol.0.0.value.to_owned()), 
			symbol.1
				.iter()
				.cloned()
				.map(|g| 
					TypeKind::Generic(GenericType { 
						identifier: g.identifier, 
						type_value: None 
					})
				)
				.collect()
		));
		new_symbol
	}
}

impl From<(&Self, &Vec<TypeKind>)> for SymbolNode {
	fn from(symbol: (&Self, &Vec<TypeKind>)) -> Self {
		let mut new_symbol = symbol.0.to_owned();
		new_symbol.0.value = Value::TypeIdentifier(TypeId(
			Box::new(symbol.0.0.value.to_owned()), 
			symbol.1.to_owned()
		));
		new_symbol
	}
}

impl AST for SymbolNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		scope.get_symbol(self)
	}
}

impl ErrorHandler for SymbolNode {
	fn get_token(&self) -> &crate::Token {
    	self.0.get_token()
	}
	fn get_context(&self) -> String {
    	format!("accessing identifier {}", self.0.value)
	}
}

impl SymbolNode {
	pub fn get_identifier_string(&self) -> &str {
		if let Value::String(s) = &self.0.value {
			s
		} else {
			panic!()
		}
	}
	pub fn new_string(string: String, meta: Meta<Flag>) -> Self {
		let mut value_node = ValueNode::from(string);
		value_node.meta = meta;
		Self(value_node)
	}
	pub fn new_generic(name: String, meta: Meta<Flag>) -> Self {
		Self(ValueNode {
			type_kind: TypeKind::new_generic(name.as_str()),
			value: Value::String(name),
			meta
		})
	}
}


impl Display for SymbolNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.0)
	}
}