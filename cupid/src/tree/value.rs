use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::{Token, TypeKind, Value, ParseNode, AST, LexicalScope, Error, ErrorHandler};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueNode {
	pub value: Value,
	pub type_kind: TypeKind,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Meta<F> {
	pub tokens: Vec<Token>,
	pub identifier: Option<Box<ValueNode>>,
	pub flags: Vec<F>,
}

impl<F> Default for Meta<F> {
	fn default() -> Self {
    	Self {
			tokens: vec![],
			identifier: None,
			flags: vec![]
		}
	}
}

impl<F> Meta<F> {
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

impl From<&Meta<()>> for Meta<Flag> {
	fn from(meta: &Meta<()>) -> Self {
    	Self {
			tokens: meta.tokens.to_owned(),
			identifier: meta.identifier.to_owned(),
			flags: vec![]
		}
	}
}

impl From<&mut ParseNode> for ValueNode {
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

impl From<(Value, &Meta<Flag>)> for ValueNode {
	fn from(value: (Value, &Meta<Flag>)) -> Self {
		Self {
			type_kind: TypeKind::infer(&value.0),
			value: value.0,
			meta: value.1.to_owned()
		}
	}
}

impl From<Value> for ValueNode {
	fn from(value: Value) -> Self {
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta::default()
		}
	}
}

impl From<String> for ValueNode {
	fn from(value: String) -> Self {
		let value = Value::String(value);
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta::default()
		}
	}
}

impl ValueNode {
	fn parse_value(node: &mut ParseNode) -> (Value, Vec<Token>) {
		let tokens = node.tokens.to_owned();
		(match node.name.as_str() {
			"boolean" => match tokens[0].source.as_str() {
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
						string.remove(0);
						string.pop();
					}
				}
				Value::String(string)
			},
			"decimal" => Value::Decimal(
				tokens[0].source.parse::<i32>().unwrap(),
				tokens[1].source.parse::<u32>().unwrap(),
			),
			"number" => Value::Integer(tokens[0].source.parse::<i32>().unwrap()),
			_ => panic!("could not parse value from {:?}", node)
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

impl AST for ValueNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.to_owned();
		if let Some(type_kind) = TypeKind::infer_from_scope(&value, scope) {
			value.type_kind = type_kind;
		}
		Ok(value)
	}
}

impl ErrorHandler for ValueNode {
	fn get_token(&self) -> &Token {
		if !self.meta.tokens.is_empty() {
    		&self.meta.tokens[0]
		} else {
			panic!("An error occurred for `{self}`, but it has no tokens to reference for position/line information")
		}
	}
	fn get_context(&self) -> String {
    	format!("{}", self.value)
	}
}