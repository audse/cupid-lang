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

impl FromParse for Result<TypeHintNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let tokens = node.collect_tokens();
		let args: Vec<Self> = node.children.iter_mut().skip(1).map(Self::from_parse).collect();
		let mut arg_items = vec![];
		for arg in args.into_iter() {
			arg_items.push(arg?);
		}
		Ok(TypeHintNode {
			identifier: node.children[0].tokens[0].source.to_owned(),
			args: arg_items,
			meta: Meta::with_tokens(tokens)
		})
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
				token_store: None,
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
				token_store: None,
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
				token_store: None,
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
		let mut symbol = SymbolNode::from(self);
		symbol.0.meta.set_token_store(scope);
		if self.is_generic() {
			return Ok((symbol, TypeKind::Generic(GenericType::new(&self.identifier, None))))
		}
		scope.get_value(
			&symbol,
			&|symbol_value| {
				let value = symbol_value.get_value(&symbol);
				if let Value::Type(type_kind) = value.value {
					Ok((symbol.to_owned(), type_kind))
				} else {
					Err((symbol.0.to_owned(), format!("expected type, found {value}"), String::new()))
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
	fn get_token<'a>(&'a self, scope: &'a mut LexicalScope) -> &'a Token {
    	if !self.meta.tokens.is_empty() {
			&self.meta.tokens[0]
		} else if let Some(token_store) = &self.meta.token_store {
			&scope.tokens[*token_store][0]
		} else {
			panic!("an error occured for type {self}, but there is no positional info")
		}
	}
	fn get_context(&self) -> String {
    	format!("accessing type {self}")
	}
}