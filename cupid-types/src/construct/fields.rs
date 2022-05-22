use crate::*;

#[derive(Debug, Clone)]
pub enum FieldSet {
	Unnamed(Vec<Type>),
	Named(Vec<TypedIdent>),
	Sum(Box<FieldSet>),
	Empty,
}

impl Default for FieldSet { fn default() -> Self { Self::Empty } }

impl FieldSet {
	pub fn unnamed(types: Vec<Type>) -> Self {
		Self::Unnamed(types)
	}
}
