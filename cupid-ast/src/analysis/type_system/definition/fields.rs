use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap)]
pub enum FieldSet {
	Unnamed(Vec<Ident>),
	Named(Vec<TypedIdent>),
	Empty,
}

impl Default for FieldSet { fn default() -> Self { Self::Empty } }
