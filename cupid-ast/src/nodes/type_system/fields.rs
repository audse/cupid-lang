use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap, Tabled)]
pub enum FieldSet {
	Unnamed(Vec<Typed<Ident>>),
	Named(Vec<(Str, Typed<Ident>)>),
	Empty,
}

impl Default for FieldSet { 
	fn default() -> Self { Self::Empty } 
}

impl FieldSet {
	pub fn iter_named(&self) -> impl Iterator<Item = &(Str, Typed<Ident>)> + '_ {
		match self {
			Self::Unnamed(_) => [].iter(),
			Self::Named(fields) => fields.iter(),
			Self::Empty => [].iter()
		}
	}
	pub fn iter_unnamed(&self) -> impl Iterator<Item = &Typed<Ident>> + '_ {
		match self {
			Self::Unnamed(fields) => fields.iter(),
			Self::Named(_) => [].iter(),
			Self::Empty => [].iter()
		}
	}
	pub fn iter_mut_named(&mut self) -> impl Iterator<Item = &mut (Str, Typed<Ident>)> + '_ {
		match self {
			Self::Unnamed(_) => [].iter_mut(),
			Self::Named(fields) => fields.iter_mut(),
			Self::Empty => [].iter_mut()
		}
	}
	pub fn iter_mut_unnamed(&mut self) -> impl Iterator<Item = &mut Typed<Ident>> + '_ {
		match self {
			Self::Unnamed(fields) => fields.iter_mut(),
			Self::Named(_) => [].iter_mut(),
			Self::Empty => [].iter_mut()
		}
	}
}