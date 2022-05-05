use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::{Token, TypeKind, Value, ParseNode, AST, RLexicalScope, Error, ErrorHandler};

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
			"char" => Value::Char(tokens[0].source.chars().next().unwrap_or('\0')),
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
			_ => panic!("{:?}", node)
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
	pub fn from_value(value: Value) -> Self {
		Self {
			type_kind: TypeKind::infer(&value),
			value,
			meta: Meta::new(vec![], None, vec![])
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
	fn resolve(&self, _scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		Ok(self.to_owned())
	}
}

impl ErrorHandler for ValueNode {
	fn get_token(&self) -> &Token {
    	&self.meta.tokens[0]
	}
	fn get_context(&self) -> String {
    	format!("{}", self.value)
	}
}