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
	fn attributes(&self) -> &Attributes;
	fn attributes_mut(&mut self) -> &mut Attributes;
	fn source(&self) -> usize {
		self.attributes().source.unwrap_or(0)
	}
	fn source_mut(&mut self) -> usize {
		self.attributes_mut().source.unwrap_or(0)
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

impl<T: UseAttributes + Default + std::fmt::Debug> UseAttributes for Typed<T> {
	fn attributes(&self) -> &Attributes {
		self.inner().attributes()
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		self.inner_mut().attributes_mut()
	}
}