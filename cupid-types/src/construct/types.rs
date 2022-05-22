#![allow(dead_code)]
use crate::*;

pub type Str = Cow<'static, str>;
pub type TypedIdent = (Str, Type);

#[derive(Debug, Clone, Default)]
pub struct Type {
	pub name: Option<Str>,
	pub params: Vec<GenericParam>,
	pub fields: FieldSet,
	pub traits: Vec<Ident>,
	pub methods: Vec<TypedIdent>,
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.params == other.params
	}
}

impl Eq for Type {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ident {
	pub name: Str,
	pub params: Vec<GenericParam>,
}

impl Type {
	pub fn primitive(name: &'static str) -> Self {
		Type {
			name: Some(Cow::Borrowed(name)),
			params: vec![],
			fields: FieldSet::Empty,
			traits: vec![],
			methods: vec![],
		}
	}
	const fn new(name: &'static str, params: Vec<GenericParam>, fields: FieldSet) -> Self {
		Self {
			name: Some(Cow::Borrowed(name)),
			params,
			fields,
			traits: vec![],
			methods: vec![],
		}
	}
	fn into_ident(self) -> Ident {
		Ident { name: self.name.unwrap(), params: self.params }
	}
}
