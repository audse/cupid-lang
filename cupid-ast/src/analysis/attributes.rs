use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Attributes {
	pub closure: usize,
	pub source: Option<usize>,
	pub generics: GenericParams,
}

pub trait UseAttributes {
	fn attributes(&mut self) -> &mut Attributes;
	fn source(&mut self) -> usize {
		self.attributes().source.unwrap_or(0)
	}
}

impl Attributes {
	pub fn new(source: usize) -> Self {
		Self {
			closure: 0,
			source: Some(source),
			generics: GenericParams::default(),
		}
	}
}