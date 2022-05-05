use crate::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BuiltinTypeNode {
	pub symbol: SymbolNode,
	pub type_kind: TypeKind,
}

impl From<&mut ParseNode> for BuiltinTypeNode {
	fn from(node: &mut ParseNode) -> Self {
		let tokens = node.children[0].tokens.to_owned();
		let name = tokens[0].source.clone();
		let type_kind = match name.as_str() {
			"bool"
			| "char"
			| "int"
			| "dec"
			| "nothing"
			| "string" => TypeKind::Primitive(PrimitiveType::new(&name)),
			"array" => {
				let generic = TypeKind::Generic(GenericType::new("e", None));
				TypeKind::Array(ArrayType { element_type: Box::new(generic) })
			},
			"map" => {
				let key_generic = TypeKind::Generic(GenericType::new("k", None));
				let value_generic = TypeKind::Generic(GenericType::new("v", None));
				TypeKind::Map(MapType { 
					key_type: Box::new(key_generic),
					value_type: Box::new(value_generic) 
				})
			},
			"fun" => {
				let generic = TypeKind::Generic(GenericType::new("r", None));
				TypeKind::Function(FunctionType { return_type: Box::new(generic) })
			},
			_ => panic!("unexpected builtin type")
		};
		Self {
			symbol: SymbolNode::new_string(name, Meta::with_tokens(tokens)),
			type_kind,
		}
	}
}

impl AST for BuiltinTypeNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let meta = Meta::with_tokens(self.symbol.0.meta.tokens.to_owned());
		let value = ValueNode::new(Value::Type(self.type_kind.to_owned()), meta);
		let symbol_value = RSymbolValue::Declaration {
			type_hint: TypeKind::Type,
			value,
			mutable: false,
		};
		scope.set_symbol(&self.symbol, &symbol_value)
	}
}
