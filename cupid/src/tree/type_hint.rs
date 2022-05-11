use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TypeKindFlag {
	Array,
	Function,
	Map,
	Primitive,
	Struct,
	Generic,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeHintNode {
	pub type_kind: SymbolNode,
	pub args: Vec<TypeHintNode>,
	pub meta: Meta<TypeKindFlag>,
}

impl From<&mut ParseNode> for TypeHintNode {
	fn from(node: &mut ParseNode) -> Self {
		use TypeKindFlag::*;
		let flag = match &*node.name {
			"array_type_hint" => Array,
			"function_type_hint" => Function,
			"map_type_hint" => Map,
			"primitive_type_hint" => Primitive,
			"struct_type_hint" => Struct,
			_ => panic!("unexpected type hint")
		};
		Self {
			type_kind: SymbolNode::from(&mut node.children[0]),
			args: node.children.iter_mut().skip(1).map(Self::from).collect(),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag])
		}
	}
}

impl AST for TypeHintNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let type_symbol: SymbolNode = self.to_symbol(scope)?;
		let mut value: ValueNode = type_symbol.resolve(scope)?;
		match &mut value.value {
			Value::Type(_) => {
				Ok(value)
			},
			_ => Err(value.error_raw(format!("expected a type, found {value} (type {})", value.type_kind)))
		}
	}
}

impl TypeHintNode {
	pub fn resolve_to_type_kind(&self, scope: &mut LexicalScope) -> Result<TypeKind, Error> {
		let value = self.resolve(scope)?;
		match value.value {
			Value::Type(type_kind) => Ok(type_kind),
			_ => Err(value.error_raw(format!("expected a type, found {value} (type {})", value.type_kind)))
		}
	}
	pub fn to_symbol(&self, scope: &mut LexicalScope) -> Result<SymbolNode, Error> {
		let mut args: Vec<TypeKind> = vec![];
		for arg in self.args.iter() {
			let arg = arg.resolve_to_type_kind(scope)?;
			args.push(arg);
		}
		Ok(SymbolNode::from((&self.type_kind, &args)))
	}
}