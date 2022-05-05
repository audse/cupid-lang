use serde::{Serialize, Deserialize};
use crate::{SymbolNode, AST, ParseNode, RLexicalScope, ValueNode, Error, Meta, TypeKind, Value, ErrorHandler, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeKindFlag {
	Array,
	Function,
	Map,
	Primitive,
	Struct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeHintNode {
	pub type_kind: SymbolNode,
	pub args: Vec<TypeHintNode>,
	pub meta: Meta<TypeKindFlag>,
}

impl From<&mut ParseNode> for TypeHintNode {
	fn from(node: &mut ParseNode) -> Self {
		use TypeKindFlag::*;
		let flag = match node.name.as_str() {
			"array_type_hint" => Array,
			"function_type_hint" => Function,
			"map_type_hint" => Map,
			"primitive_type_hint" => Primitive,
			"struct_type_hint" => Struct,
			_ => panic!()
		};
		Self {
			type_kind: SymbolNode::from(&mut node.children[0]),
			args: node.children.iter_mut().skip(1).map(|t| Self::from(t)).collect(),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag])
		}
	}
}

impl AST for TypeHintNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		// 1. get from scope
		// 2. apply args to generics
		let value = self.type_kind.resolve(scope)?;
		match value.value {
			Value::Type(_) => {
				Ok(value)
			},
			_ => Err(value.error_raw("not a type"))
		}
	}
}

impl TypeHintNode {
	pub fn resolve_to_type_kind(&self, scope: &mut RLexicalScope) -> Result<TypeKind, Error> {
		let value = self.resolve(scope)?;
		match value.value {
			Value::Type(type_kind) => Ok(type_kind),
			_ => Err(value.error_raw("not a type"))
		}
	}
}