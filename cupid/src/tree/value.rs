use crate::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct ValueNode {
	pub value: Value,
	pub type_hint: Option<TypeHintNode>,
	pub meta: Meta<Flag>,
}

impl PartialEq for ValueNode {
	fn eq(&self, other: &Self) -> bool {
    	self.value == other.value
	}
}

impl Eq for ValueNode {}

impl Hash for ValueNode {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state);
	}
}

impl Display for ValueNode {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.value)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Flag {
	Return,
	Break,
	Continue,
	Pointer,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Meta<F> {
	pub tokens: Vec<Token>,
	pub token_store: Option<usize>,
	pub identifier: Option<Box<ValueNode>>,
	pub flags: Vec<F>,
}

impl<F> Default for Meta<F> {
	fn default() -> Self {
    	Self {
			tokens: vec![],
			token_store: None,
			identifier: None,
			flags: vec![]
		}
	}
}

impl<F> Meta<F> {
	pub fn new(tokens: Vec<Token>, identifier: Option<Box<ValueNode>>, flags: Vec<F>) -> Self {
		Self {
			tokens,
			token_store: None,
			identifier,
			flags
		}
	}
	pub fn with_tokens(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			token_store: None,
			identifier: None,
			flags: vec![]
		}
	}
	pub fn set_token_store(&mut self, scope: &mut LexicalScope) {
		self.token_store = Some(scope.push_tokens(std::mem::take(&mut self.tokens)));
	}
}

impl<F: std::fmt::Debug> std::fmt::Debug for Meta<F> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Meta")
		.field("tokens", &format_args!("{:?}", self.tokens))
		.field("token_store", &self.token_store)
			.field("identifier", &self.identifier)
			.field("flags",&format_args!("{:?}", self.flags))
			.finish()
	}
}

impl<F, T> From<&Meta<F>> for Meta<T> {
	fn from(meta: &Meta<F>) -> Self {
		Self {
			tokens: meta.tokens.to_owned(),
			token_store: meta.token_store.to_owned(),
			identifier: meta.identifier.to_owned(),
			flags: vec![]
		}
	}
}

impl FromParse for Result<ValueNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let (value, tokens) = ValueNode::parse_value(node)?;
		let mut node = ValueNode {
			type_hint: None,
			value,
			meta: Meta {
				tokens,
				token_store: None,
				identifier: None,
				flags: vec![],
			},
		};
		node.type_hint = TypeKind::infer_id(&node);
		Ok(node)
	}
}

impl From<(Value, &Meta<Flag>)> for ValueNode {
	fn from(value: (Value, &Meta<Flag>)) -> Self {
		let mut node = Self {
			type_hint: None,
			value: value.0,
			meta: value.1.to_owned()
		};
		node.type_hint = if let Some(mut type_hint) = TypeKind::infer_id(&node) {
			type_hint.meta.tokens = node.meta.tokens.to_owned();
			Some(type_hint)
		} else {
			None
		};
		node
	}
}
impl From<(Value, Meta<Flag>)> for ValueNode {
	fn from(value: (Value, Meta<Flag>)) -> Self {
		let mut node = Self {
			type_hint: None,
			value: value.0,
			meta: value.1
		};
		node.type_hint = TypeKind::infer_id(&node);
		node
	}
}

impl ValueNode {
	fn parse_value(node: &mut ParseNode) -> Result<(Value, Vec<Token>), Error> {
		let tokens = node.tokens.to_owned();
		Ok((match &*node.name {
			"boolean" => match &*tokens[0].source {
				"true" => Value::Boolean(true),
				"false" => Value::Boolean(false),
				_ => panic!("booleans can only be 'true' or 'false'"),
			},
			"none" => Value::None,
			"char" => {
				if tokens.len() == 2 {
					Value::Char(tokens[1].source.chars().next().unwrap_or('\0'))
				} else {
					let chars = [
						tokens[1].source.chars().next().unwrap_or_else(|| panic!("expected char")),
						tokens[2].source.chars().next().unwrap_or_else(|| panic!("expected char")),
					];
					let c = match format!("{}{}", chars[0], chars[1]).as_str() {
						r"\n" => '\n',
						r"\t" => '\t',
						r"\r" => '\r',
						r"\s" => ' ',
						c => return Err(Error::from_token(
							&node.tokens[1], 
							&format!("could not parse char from `{c}`"), 
							""
						))
					};
					Value::Char(c)
				}
			},
			"pointer" => Value::Pointer(Box::new(
				Result::<SymbolNode, Error>::from_parse(&mut node.children[0])?
			)),
			"string"
			| "identifier"
			| "builtin_function"
			| "self"
			| "array_kw"
			| "map_kw"
			| "fun_kw" => {
				let mut string = tokens[0].source.clone();
				if let Some(first) = string.chars().next() {
					if is_quote(first) {
						string = Cow::Owned(string[1..string.len()-1].to_string());
					}
				}
				Value::String(string)
			},
			"decimal" => Value::Decimal(
				tokens[0].source.parse::<i32>().unwrap(),
				tokens[1].source.parse::<u32>().unwrap(),
			),
			"number" => Value::Integer(tokens[0].source.parse::<i32>().unwrap()),
			_ => panic!("could not parse value: {node:?}")
		}, tokens))
	}
	pub fn set_meta_identifier(&mut self, identifier: &Self) {
		self.meta.identifier = Some(Box::new(identifier.to_owned()));
	}
	pub fn new(value: Value, meta: Meta<Flag>) -> Self {
		Self::from((value, meta))
	}
	pub fn new_none() -> Self {
		let value = Value::None;
		Self::from((value, Meta::default()))
	}
	pub fn as_assignment(&self) -> SymbolValue {
		SymbolValue::Assignment { value: self.to_owned() }
	}
	pub fn into_declaration(self, mutable: bool) -> SymbolValue {
		SymbolValue::Declaration { 
			type_hint: if let Some(type_hint) = self.type_hint.to_owned() {
				Some(type_hint)
			} else {
				TypeKind::infer_id(&self)
			}, 
			mutable, 
			value: self
		}
	}
	pub fn into_declaration_node(self, symbol: SymbolNode, type_hint: TypeHintNode, mutable: bool) -> DeclarationNode {
		DeclarationNode {
			symbol,
			type_hint,
			mutable,
			meta: Meta::<()>::from(&self.meta),
			value: BoxAST::new(self),
		}
	}
	pub fn get_property(&self, property: &ValueNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		// try to get direct property from arrays/maps
		if let Ok(mut value) = self.value.get_property(property) {
			if let Some(type_hint) = &self.type_hint {
				let type_kind: TypeKind = type_hint.resolve_to(scope)?;
				let new_type_hint = match type_kind {
					TypeKind::Struct(s) => s.members.iter().find(|(member, _)| member == property).map(|(_, t)| t).unwrap_or(type_hint).to_owned(),
					TypeKind::Array(a) => a.element_type,
					TypeKind::Map(m) => m.value_type,
					_ => type_hint.to_owned()
				};
				value.type_hint = Some(new_type_hint)
			}
			return Ok(value);
		}
		Err(property.error(format!("could not find property `{property}` in `{self}`"), scope))
	}
	pub fn get_method(&self, method: &SymbolNode, scope: &mut LexicalScope) -> Result<(Option<(Implementation, Option<Implementation>)>, FunctionNode), Error> {
		if let Some(type_hint) = &self.type_hint {
			// try to get implemented method or trait of variable
			let type_hint: TypeHintNode = type_hint.resolve_to(scope)?;
			let mut type_kind: TypeKind = type_hint.resolve_to(scope)?;
			
			if let Some((type_implement, trait_implement, function)) = type_kind.get_trait_function(&method, scope) {
				return Ok((Some((type_implement, trait_implement)), function));
			}
		} else if let Ok(property) = self.get_property(&method.0, scope) {
			if let Value::Function(function) = property.value {
				return Ok((None, function));
			}
		} else {
			let value = self.resolve(scope)?;
			
			// try to get associated method of INFERRED type
			if let Some(inferred_type_kind) = TypeKind::infer_id(&value) {
				let mut inferred_type: TypeKind = inferred_type_kind.resolve_to(scope)?;
				if let Some((type_implement, trait_implement, function)) = inferred_type.get_trait_function(&method, scope) {
					return Ok((Some((type_implement, trait_implement)), function));
				}
			}
			
			if let Value::Type(mut type_kind) = value.value {
				// try to get associated method or trait of type kinds
				if let Some((type_implement, trait_implement, function)) = type_kind.get_trait_function(&method, scope) {
					return Ok((Some((type_implement, trait_implement)), function));
				}
			}
		}
		Err(method.error("could not find associated method", scope))
	}
}

impl AST for ValueNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.to_owned();
		
		// Move token information to scope to avoid unecessarily passing around
		// and cloning source info
		value.meta.set_token_store(scope);
		
		value.type_hint = TypeKind::infer_id(&value);
		Ok(value)
	}
}

impl ErrorHandler for ValueNode {
	fn get_token<'a>(&'a self, scope: &'a mut LexicalScope) -> &'a Token {
		if !self.meta.tokens.is_empty() {
    		return &self.meta.tokens[0]
		} else if let Some(token_store) = &self.meta.token_store {
			if !scope.tokens[*token_store].is_empty() {
				return &scope.tokens[*token_store][0]
			}
		}
		panic!("An error occurred for `{self}`, but there are no tokens to reference for position/line information")
	}
	fn get_context(&self) -> String {
    	format!("{}", self.value)
	}
}


impl std::fmt::Debug for ValueNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let type_hint_str = if let Some(t) = &self.type_hint {
			format!("{:#?}", t)
		} else {
			"None".to_string()
		};
		f.debug_struct("ValueNode")
			.field("value", &format_args!("{:?}", self.value))
			.field("type_hint", &format_args!("{type_hint_str}"))
			.field("meta", &self.meta)
			.finish()
	}
}

fn is_quote(c: char) -> bool {
	c == '"' || c =='\''
}