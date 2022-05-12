use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode(pub ValueNode);

impl From<&mut ParseNode> for SymbolNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut value_node = ValueNode::from(node);
		// symbols do not have their own type
		value_node.type_hint = None;
    	Self(value_node)
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
}


impl Display for SymbolNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.0)
	}
}