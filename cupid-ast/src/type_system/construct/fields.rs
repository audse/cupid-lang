use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldSet {
	Unnamed(Vec<Ident>),
	Named(Vec<TypedIdent>),
	Sum(Box<FieldSet>),
	Empty,
}

impl Default for FieldSet { fn default() -> Self { Self::Empty } }

impl FieldSet {
	pub fn unnamed(types: Vec<Ident>) -> Self {
		Self::Unnamed(types)
	}
	pub fn sum_named(types: Vec<TypedIdent>) -> Self {
		Self::Sum(Box::new(Self::Named(types)))
	}
	pub fn sum_unnamed(types: Vec<Ident>) -> Self {
		Self::Sum(Box::new(Self::Unnamed(types)))
	}
}
