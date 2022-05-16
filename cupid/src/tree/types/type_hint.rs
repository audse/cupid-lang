use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeFlag {
	Alias,
	Array,
	Function,
	Generic,
	Map,
	Primitive,
	Struct,
	Sum,
	Trait,
	
	Inferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeHintNode {
	pub identifier: Cow<'static, str>,
	pub args: Vec<TypeHintNode>,
	pub meta: Meta<TypeFlag>,
}

impl From<&mut ParseNode> for TypeHintNode {
	fn from(node: &mut ParseNode) -> Self {
		let tokens = node.collect_tokens();
		Self {
			identifier: node.children[0].tokens[0].source.to_owned(),
			args: node.children.iter_mut().skip(1).map(Self::from).collect(),
			meta: Meta::with_tokens(tokens)
		}
	}
}

impl AST for TypeHintNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let (mut symbol, type_kind) = self.resolve_to(scope)?;
		symbol.0.value = Value::Type(type_kind);
		Ok(symbol.0)
	}
}

impl TypeHintNode {
	pub fn new(identifier: Cow<'static, str>, flags: Vec<TypeFlag>, args: Vec<Self>, tokens: Vec<Token>) -> Self {
		Self {
			identifier,
			args,
			meta: Meta {
				tokens,
				flags,
				identifier: None
			}
		}
	}
	pub fn generic(identifier: Cow<'static, str>, tokens: Vec<Token>) -> Self {
		Self {
			identifier,
			args: vec![],
			meta: Meta {
				tokens,
				flags: vec![TypeFlag::Generic],
				identifier: None
			}
		}
	}
	pub fn generic_arg(identifier: Cow<'static, str>, arg: Option<TypeHintNode>, tokens: Vec<Token>) -> Self {
		Self {
			identifier,
			args: if let Some(arg) = arg { vec![arg] } else { vec![] },
			meta: Meta {
				tokens,
				flags: vec![TypeFlag::Generic],
				identifier: None
			}
		}
	}
	pub fn is_generic(&self) -> bool {
		self.meta.flags.contains(&TypeFlag::Generic)
	}
}

impl ResolveTo<(SymbolNode, TypeKind)> for TypeHintNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<(SymbolNode, TypeKind), Error> {
		let symbol = SymbolNode::from(self);
		scope.get_value(
			&symbol,
			&|symbol_value| {
				if let Value::Type(type_kind) = symbol_value.get_value(&symbol).value {
					Ok((symbol.to_owned(), type_kind))
				} else {
					Err(symbol.error_raw("expected type"))
				}
			}
		)
	}
}

impl ResolveTo<TypeKind> for TypeHintNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<TypeKind, Error> {
		let (_, type_kind) = self.resolve_to(scope)?;
		Ok(type_kind)
	}
}

impl ResolveTo<TypeHintNode> for TypeHintNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<TypeHintNode, Error> {
		let (_, type_kind) = self.resolve_to(scope)?;
		if let TypeKind::Generic(generic) = type_kind {
			if let Some(value) = &generic.type_value {
				Ok(value.to_owned())
			} else {
				Ok(self.to_owned())
			}
		} else {
			Ok(self.to_owned())
		}
	}
}

impl From<&TypeHintNode> for SymbolNode {
	fn from(node: &TypeHintNode) -> Self {
		Self(ValueNode {
			value: Value::TypeHint(node.to_owned()),
			type_hint: None,
			meta: Meta::<Flag>::from(&node.meta)
		})
	}
}

impl PartialEq for TypeHintNode {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier 
		&& self.args.len() == other.args.len()
		&& self.args
			.iter()
			.enumerate()
			.all(|(i, arg)| 
				arg.is_generic() 
				|| other.args[i].is_generic() 
				|| arg == &other.args[i]
			)
	}
}

impl Eq for TypeHintNode {}

impl Hash for TypeHintNode {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
	}
}

impl Display for TypeHintNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let identifier = if self.is_generic() { 
			format!("<{}>", self.identifier) 
		} else { 
			self.identifier.to_string() 
		};
		let args: Vec<String> = self.args.iter().map(|a| a.to_string()).collect();
		let args: String = if args.is_empty() { 
			String::new() 
		} else { 
			format!(" [{}]", args.join(", ")) 
		};
		write!(f, "{identifier}{args}")
	}
}

impl ErrorHandler for TypeHintNode {
	fn get_token(&self) -> &Token {
    	if !self.meta.tokens.is_empty() {
			&self.meta.tokens[0]
		} else {
			panic!("an error occured for type {self}, but there is no positional info")
		}
	}
	fn get_context(&self) -> String {
    	format!("accessing type {self}")
	}
}