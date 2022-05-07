use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::{ParseNode, ValueNode, AST, Error, RLexicalScope, ErrorHandler, RScope, Value, Meta, Flag};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode(pub ValueNode);

impl From<&mut ParseNode> for SymbolNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self(ValueNode::from(node))
	}
}

impl AST for SymbolNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		scope.get_symbol(self)
	}
}

impl ErrorHandler for SymbolNode {
	fn get_token(&self) -> &crate::Token {
    	&self.0.meta.tokens.get(0).unwrap_or_else(|| panic!("no token for `{self}`"))
	}
	fn get_context(&self) -> String {
    	format!("accessing identifier {}", self.0.value)
	}
}

impl SymbolNode {
	pub fn get_identifier_string(&self) -> &str {
		if let Value::String(s) = &self.0.value {
			&s
		} else {
			panic!()
		}
	}
	pub fn new_string(string: String, meta: Meta<Flag>) -> Self {
		Self(ValueNode::new(Value::String(string), meta))
	}
}


impl Display for SymbolNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.0)
	}
}