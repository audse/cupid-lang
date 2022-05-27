use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct Attributes {
	pub closure: usize,
	#[tabled(display_with = "fmt_src")]
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

fmt_option_fn!(fmt_src: usize);