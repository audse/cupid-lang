use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
	pub AttributesBuilder => pub Attributes {
		pub closure: usize,
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

impl UseAttributes for Attributes {
	fn attributes(&self) -> &Attributes {
		self
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		self
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

impl<'ast, T: UseAttributes> UseAttributes for &'ast T {
	fn attributes(&self) -> &Attributes {
		(*self).attributes()
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		todo!()
	}
}