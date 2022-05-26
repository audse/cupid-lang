use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode(pub ValueNode);

impl FromParse for Result<SymbolNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let value_node = Result::<ValueNode, Error>::from_parse(node)?;
    	Ok(SymbolNode(value_node))
	}
}

impl AST for SymbolNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = scope.get_symbol(self)?;
		value.meta.set_token_store(scope);
		if let Value::Pointer(pointer) = &value.value {
			scope.get_symbol(&*pointer)
		} else {
			Ok(value)
		}
	}
	fn as_symbol(&self) -> Option<&SymbolNode> { Some(&self) }
}

impl ErrorHandler for SymbolNode {
	fn get_token<'a>(&'a self, scope: &'a mut LexicalScope) -> &'a crate::Token {
    	self.0.get_token(scope)
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
			panic!("{self:?}")
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