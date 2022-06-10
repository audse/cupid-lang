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

pub trait UseClosure: UseAttributes {
	fn set_closure(&mut self, scope: &Env) {
		self.attributes_mut().closure = scope.current_closure;
	}
	fn set_closure_to(&mut self, closure: usize) {
		self.attributes_mut().closure = closure;
	}
	fn closure(&self) -> usize {
		self.attributes().closure
	}
	fn use_closure(&self, scope: &mut Env) {
		scope.use_closure(self.closure(), fmt_type!(Self));
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

pub trait Trace {}