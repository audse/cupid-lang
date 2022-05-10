use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueNode<'src> {
	pub value: Value<'src>,
	pub type_kind: TypeKind<'src>,
	pub meta: Meta<'src, Flag>,
}

impl<'src> PartialEq for ValueNode<'src> {
	fn eq(&self, other: &Self) -> bool {
    	self.value == other.value
	}
}

impl<'src> Eq for ValueNode<'src> {}

impl<'src> Hash for ValueNode<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state);
	}
}

impl<'src> Display for ValueNode<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.value)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Flag {
	Return,
	Break,
	Continue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Meta<'src, F> {
	pub tokens: Vec<Token<'src>>,
	pub identifier: Option<Box<ValueNode<'src>>>,
	pub flags: Vec<F>,
}

impl<'src, F> Default for Meta<'src, F> {
	fn default() -> Self {
    	Self {
			tokens: vec![],
			identifier: None,
			flags: vec![]
		}
	}
}

impl<'src, F> Meta<'src, F> {
	pub fn new(tokens: Vec<Token>, identifier: Option<Box<ValueNode>>, flags: Vec<F>) -> Self {
		Self {
			tokens,
			identifier,
			flags
		}
	}
	pub fn with_tokens(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			identifier: None,
			flags: vec![]
		}
	}
}

impl<'src> From<&Meta<'src, ()>> for Meta<'src, Flag> {
	fn from(meta: &Meta<()>) -> Self {
    	Self {
			tokens: meta.tokens.to_owned(),
			identifier: meta.identifier.to_owned(),
			flags: vec![]
		}
	}
}

impl<'src> From<&mut ParseNode<'src>> for ValueNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		let (value, tokens) = Self::parse_value(node);
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta {
				tokens,
				identifier: None,
				flags: vec![],
			},
		}
	}
}

impl<'src> From<(Value<'src>, &Meta<'src, Flag>)> for ValueNode<'src> {
	fn from(value: (Value, &Meta<Flag>)) -> Self {
		Self {
			type_kind: TypeKind::infer(&value.0),
			value: value.0,
			meta: value.1.to_owned()
		}
	}
}

impl<'src> From<Value<'src>> for ValueNode<'src> {
	fn from(value: Value) -> Self {
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta::default()
		}
	}
}

impl<'src> From<Cow<'src, str>> for ValueNode<'src> {
	fn from(value: Cow<'src, str>) -> Self {
		let value = Value::String(value);
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta::default()
		}
	}
}

impl<'src> ValueNode<'src> {
	fn parse_value(node: &mut ParseNode) -> (Value<'src>, Vec<Token<'src>>) {
		let tokens = node.tokens.to_owned();
		(match &*node.name {
			"boolean" => match &*tokens[0].source {
				"true" => Value::Boolean(true),
				"false" => Value::Boolean(false),
				_ => panic!("booleans can only be 'true' or 'false'"),
			},
			"none" => Value::None,
			"char" => Value::Char(tokens[1].source.chars().next().unwrap_or('\0')),
			"string"
			| "identifier"
			| "self"
			| "array_kw"
			| "map_kw"
			| "fun_kw" => {
				let mut string = tokens[0].source.clone();
				if let Some(first) = string.chars().next() {
					if first == '"' || first == '\'' { 
						string = Cow::Owned(string[1..string.len()].to_string());
					}
				}
				Value::String(string)
			},
			"decimal" => Value::Decimal(
				tokens[0].source.parse::<i32>().unwrap(),
				tokens[1].source.parse::<u32>().unwrap(),
			),
			"number" => Value::Integer(tokens[0].source.parse::<i32>().unwrap()),
			_ => panic!("could not parse value")
		}, tokens)
	}
	pub fn set_meta_identifier(&mut self, identifier: &Self) {
		self.meta.identifier = Some(Box::new(identifier.to_owned()));
	}
	pub fn new(value: Value, meta: Meta<Flag>) -> Self {
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta,
		}
	}
	pub fn new_none() -> Self {
		let none = Value::None;
		Self {
			type_kind: TypeKind::infer(&none),
			value: none,
			meta: Meta::new(vec![], None, vec![])
		}
	}
}

impl<'src> AST for ValueNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.to_owned();
		if let Some(type_kind) = TypeKind::infer_from_scope(&value, scope) {
			value.type_kind = type_kind;
		}
		Ok(value)
	}
}

impl<'src> ErrorHandler for ValueNode<'src> {
	fn get_token(&self) -> &Token {
		if !self.meta.tokens.is_empty() {
    		&self.meta.tokens[0]
		} else {
			panic!("An error occurred for `{self:?}`, but there are no tokens to reference for position/line information")
		}
	}
	fn get_context(&self) -> String {
    	format!("{}", self.value)
	}
}