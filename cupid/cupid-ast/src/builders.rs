use crate::*;

impl TypeBuilder {
	pub fn name_str(mut self, name: &'static str) -> Self {
		self.name.name = name.into();
		self
	}
	pub fn add_generic(mut self, index: usize, generic: Typed<Ident>) -> Self {
		self.name.attributes.generics.0.insert(index, generic);
		self
	}
	pub fn primitive(self, name: &'static str) -> Self {
		self.name_str(name)
			.base_type(BaseType::Primitive(name.into()))
	}
	pub fn base_primitive(mut self, name: &'static str) -> Self {
		self.base_type = BaseType::Primitive(name.into());
		self
	}
}

impl IdentBuilder {
	pub fn one_generic(mut self, generic: Typed<Ident>) -> Self {
		self.attributes.generics = GenericList(vec![generic]);
		self
	}
}