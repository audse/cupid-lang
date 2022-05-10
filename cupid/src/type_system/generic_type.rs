use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType<'src> {
	pub identifier: Cow<'src, str>,
	pub type_value: Option<Box<TypeKind<'src>>>
}

impl<'src> GenericType<'src> {
	pub const fn new_const(identifier: &'static str) -> Self {
		Self { identifier: Cow::Borrowed(identifier), type_value: None }
	}
	pub fn new(identifier: &str, type_value: Option<Box<TypeKind>>) -> Self {
		Self { identifier: Cow::Owned(identifier.to_string()), type_value }
	}
}

impl<'src> From<&SymbolNode<'src>> for GenericType<'src> {
	fn from(symbol: &SymbolNode) -> Self {
		if let Value::String(string) = &symbol.0.value {
			Self {
				identifier: (*string).to_owned(),
				type_value: None
			}
		} else {
			Self::new("t", None)
		}
	}
}

impl<'src> From<&'src str> for GenericType<'src> {
	fn from(string: &'src str) -> Self {
		Self { identifier: string.into(), type_value: None }
	}
}

impl<'src> Into<TypeKind<'src>> for GenericType<'src> {
	fn into(self) -> TypeKind<'src> { TypeKind::Generic(self) }
}
impl<'src> Into<Value<'src>> for GenericType<'src> {
	fn into(self) -> Value<'src> { Value::Type(self.into()) }
}
impl<'src> Into<ValueNode<'src>> for GenericType<'src> {
	fn into(self) -> ValueNode<'src> { ValueNode::from(Value::from(self.into())) }
}

impl<'src> Type for GenericType<'src> {}

impl<'src> PartialEq for GenericType<'src> {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}

impl<'src> Eq for GenericType<'src> {}

impl<'src> Hash for GenericType<'src> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
		self.type_value.hash(state);
	}
}

impl<'src> Display for GenericType<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let type_value = if let Some(type_value) = &self.type_value {
			format!(": {}", type_value)
		} else {
			String::new()
		};
		write!(f, "<{}{}>", self.identifier, type_value)
	}
}