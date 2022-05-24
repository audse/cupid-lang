use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Attributes {
	pub closure: usize,
	pub source: Option<usize>,
	pub generics: Vec<GenericParam>
}

pub trait UseAttributes {
	fn attributes(&mut self) -> &mut Attributes;
}

impl Attributes {
	pub fn new(source: usize) -> Self {
		Self {
			closure: 0,
			source: Some(source),
			generics: vec![]
		}
	}
}