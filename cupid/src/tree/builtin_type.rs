use crate::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BuiltinTypeNode<'src> {
	pub symbol: SymbolNode<'src>,
	pub type_kind: TypeKind<'src>,
	pub generics: Vec<GenericType<'src>>,
}

impl<'src> From<&mut ParseNode<'src>> for BuiltinTypeNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		let mut tokens = node.tokens.to_owned();
		let name = &mut tokens[1].source;
		let (type_kind, generics) = match &**name {
			"bool"
			| "char"
			| "int"
			| "dec"
			| "nothing"
			| "string" => (TypeKind::new_primitive(&name), vec![]),
			"array" => (TypeKind::new_array(TypeKind::new_generic("e")), vec![GenericType::from("e")]),
			"map" => (
				TypeKind::new_map(TypeKind::new_generic("k"), TypeKind::new_generic("v")),
				vec![GenericType::from("k"), GenericType::from("v")]
			),
			"fun" => (TypeKind::new_function(), vec![]),
			_ => panic!("unexpected builtin type")
		};
		Self {
			symbol: SymbolNode::new_string(std::mem::take(&mut *name), Meta::with_tokens(tokens)),
			generics,
			type_kind,
		}
	}
}

impl<'src> AST for BuiltinTypeNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let meta = Meta::with_tokens(self.symbol.0.meta.tokens.to_owned());
		
		let symbol: (&SymbolNode, &Vec<GenericType>) = (&self.symbol, &self.generics);
		let symbol = SymbolNode::from(symbol);
		
		let value = ValueNode::new(Value::Type(self.type_kind.to_owned()), meta);
		let symbol_value = SymbolValue::Declaration {
			type_hint: TypeKind::Type,
			value,
			mutable: false,
		};
		scope.set_symbol(&symbol, symbol_value)
	}
}
