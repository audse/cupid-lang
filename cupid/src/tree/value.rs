use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl<F, T> From<&Meta<F>> for Meta<T> {
	fn from(meta: &Meta<F>) -> Self {
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
		let mut node = Self {
			type_hint: None,
			value,
			meta: Meta {
				tokens,
				identifier: None,
				flags: vec![],
			},
		};
		node.type_hint = TypeKind::infer_id(&node);
		node
	}
}

impl From<(Value, &Meta<Flag>)> for ValueNode {
	fn from(value: (Value, &Meta<Flag>)) -> Self {
		let mut node = Self {
			type_hint: None,
			value: value.0,
			meta: value.1.to_owned()
		};
		node.type_hint = TypeKind::infer_id(&node);
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
	fn parse_value(node: &mut ParseNode) -> (Value, Vec<Token>) {
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
			_ => panic!("could not parse value")
		}, tokens)
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
}

impl AST for ValueNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.to_owned();
		value.type_hint = TypeKind::infer_id(&value);
		Ok(value)
	}
}

impl ErrorHandler for ValueNode {
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

fn is_quote(c: char) -> bool {
	c == '"' || c =='\''
}