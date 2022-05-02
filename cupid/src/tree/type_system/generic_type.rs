use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use crate::{TypeKind, Type, Symbol, LexicalScope, SymbolFinder, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
	pub identifier: Cow<'static, str>,
	pub type_value: Option<Box<TypeKind>>
}

impl GenericType {
	pub const fn new_const(identifier: &'static str) -> Self {
		Self { identifier: Cow::Borrowed(identifier), type_value: None }
	}
	pub fn new(identifier: &str, type_value: Option<Box<TypeKind>>) -> Self {
		Self { identifier: Cow::Owned(identifier.to_string()), type_value }
	}
}

impl Type for GenericType {}

impl PartialEq for GenericType {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
	}
}

impl Eq for GenericType {}

impl Hash for GenericType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
		self.type_value.hash(state);
	}
}

impl Display for GenericType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "<{}>", self.identifier)
	}
}

pub trait UseGenerics {
	fn get_generics(&self) -> &[Symbol];
	fn define_generics(&self, scope: &mut LexicalScope) {
		let generics: Vec<(&Symbol, GenericType)> = self.get_generics()
			.iter()
			.map(|g| (g, GenericType::new(&g.get_identifier(), None)))
			.collect();
		for (symbol, generic) in generics {
			scope.define_type(symbol, TypeKind::Generic(generic));
		}
	}
	fn resolve_generics(&self) -> Vec<GenericType> {
		self.get_generics()
			.iter()
			.map(|g| GenericType::new(&g.get_identifier(), None))
			.collect()
	}
	fn convert_primitives_to_generics(&self, type_value: &mut Value) -> Result<(), ()> {
		match *type_value {
			Value::Type(ref mut type_kind) => {
				type_kind.convert_primitives_to_generics(&self.resolve_generics());
				Ok(())
			},
			_ => Err(())
		}
	}
}