use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Trait {
	pub name: Str,
	pub params: Vec<GenericParam>,
	pub methods: Vec<TypedIdent>,
	pub bounds: Vec<Ident>,
}