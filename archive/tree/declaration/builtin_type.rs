use crate::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BuiltinTypeNode {
	pub type_hint: TypeHintNode,
	pub type_kind: TypeKind,
}

fn generic(name: &'static str, tokens: &Vec<Token>) -> TypeHintNode {
	TypeHintNode::generic(name.into(), tokens.to_owned())
}

impl FromParse for Result<BuiltinTypeNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let tokens = node.tokens.to_owned();
		let name = tokens[1].source.to_owned();
		let (type_hint, type_kind) = match &*name {
			"bool"
			| "char"
			| "int"
			| "dec"
			| "nothing"
			| "string" => (
				TypeHintNode::new(name.to_owned(), vec![TypeFlag::Primitive], vec![], tokens),
				TypeKind::new_primitive(name)
			),
			"array" => (
				TypeHintNode::new(name, vec![TypeFlag::Array], vec![generic("e", &tokens)], tokens),
				TypeKind::new_array(None)
			),
			"map" => (
				TypeHintNode::new(name, vec![TypeFlag::Map], vec![generic("k", &tokens), generic("v", &tokens)], tokens),
				TypeKind::new_map(None)
			),
			"fun" => (
				TypeHintNode::new(name, vec![TypeFlag::Function], vec![generic("r", &tokens)], tokens),
				TypeKind::new_function()
			),
			_ => return Err(Error::from_token(&tokens[0], "unexpected builtin type", "defining builtin type"))
		};
		Ok(BuiltinTypeNode {
			type_hint,
			type_kind,
		})
	}
}

impl AST for BuiltinTypeNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let symbol = SymbolNode::from(&self.type_hint);
		let value = SymbolValue::Declaration {
			type_hint: None,
			value: ValueNode {
				type_hint: None,
				value: Value::Type(self.type_kind.to_owned()),
				meta: Meta::<Flag>::from(&self.type_hint.meta)
			},
			mutable: false,
		};
		scope.set_symbol(&symbol, value)
	}
}

impl Display for BuiltinTypeNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}
