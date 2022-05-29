use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub AttributesBuilder => pub Attributes {
		pub closure: usize,
		#[tabled(display_with = "fmt_option")]
		pub source: Option<usize>,
		pub generics: GenericList,
	}
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
			generics: GenericList::default(),
		}
	}
}