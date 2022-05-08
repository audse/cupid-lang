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
		let flag = match node.name.as_str() {
			"array_type_hint" => Array,
			"function_type_hint" => Function,
			"map_type_hint" => Map,
			"primitive_type_hint" => Primitive,
			"struct_type_hint" => Struct,
			_ => panic!("{}", node.name)
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
		// 1. get from scope
		let mut value = self.type_kind.resolve(scope)?;
		
		let mut args: Vec<TypeKind> = vec![];
		for arg in self.args.iter() {
			let arg = arg.resolve_to_type_kind(scope)?;
			args.push(arg);
		}
		
		// 2. apply args to generics
		match &mut value.value {
			Value::Type(ref mut type_value) => {
				if let Err(_) = type_value.apply_args(args) {
					// return Err(value.error_raw(e))
				}
				Ok(value)
			},
			_ => Err(value.error_raw("not a type"))
		}
	}
}

impl TypeHintNode {
	pub fn resolve_to_type_kind(&self, scope: &mut LexicalScope) -> Result<TypeKind, Error> {
		let value = self.resolve(scope)?;
		match value.value {
			Value::Type(type_kind) => Ok(type_kind),
			_ => Err(value.error_raw("not a type"))
		}
	}
}