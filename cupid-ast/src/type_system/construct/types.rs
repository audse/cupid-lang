#![allow(dead_code)]
use crate::*;

pub type Str = Cow<'static, str>;
pub type TypedIdent = (Str, Ident);

#[derive(Debug, Clone, Default, Hash)]
pub struct Type {
	pub name: Option<Str>,
	pub generics: Vec<GenericParam>,
	pub fields: FieldSet,
	pub traits: Vec<Ident>,
	pub methods: Vec<Type>,
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.generics == other.generics
	}
}

impl Eq for Type {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Ident {
	pub name: Str,
	pub generics: Vec<GenericParam>,
}

impl Type {
	pub fn primitive(name: &'static str) -> Self {
		Type {
			name: Some(Cow::Borrowed(name)),
			generics: vec![],
			fields: FieldSet::Empty,
			traits: vec![],
			methods: vec![],
		}
	}
	const fn new(name: &'static str, generics: Vec<GenericParam>, fields: FieldSet) -> Self {
		Self {
			name: Some(Cow::Borrowed(name)),
			generics,
			fields,
			traits: vec![],
			methods: vec![],
		}
	}
	pub fn into_ident(self) -> Ident {
		Ident { name: self.name.unwrap(), generics: self.generics }
	}
	pub fn nothing() -> Self {
		NOTHING.to_owned()
	}
}