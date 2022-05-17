use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
	fn as_symbol(&self) -> Option<&SymbolNode> { Some(&self) }
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
	pub fn from_value_and_tokens(value: Value, tokens: Vec<Token>) -> Self {
		let mut symbol = Self(ValueNode {
			value,
			type_hint: None,
			meta: Meta::with_tokens(tokens)
		});
		symbol.0.type_hint = TypeKind::infer_id(&symbol.0);
		symbol
	}
}


impl Display for SymbolNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.0)
	}
}

impl std::fmt::Debug for SymbolNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Symbol({:?})", self.0.value)
	}
}