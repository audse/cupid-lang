use crate::*;

impl TypeBuilder {
	pub fn name_str(mut self, name: &'static str) -> Self {
		self.name.name = name.into();
		self
	}
	pub fn add_generic(mut self, generic: Typed<Ident>) -> Self {
		self.name.attributes.generics.push(generic);
		self
	}
	pub fn generics(mut self, generics: GenericList) -> Self {
		self.name.attributes.generics = generics;
		self
	}
	pub fn add_generic_at(mut self, index: usize, generic: Typed<Ident>) -> Self {
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
	pub fn base_array(self) -> Self { self.base_type(BaseType::Array) }
	pub fn base_fun(self) -> Self { self.base_type(BaseType::Function) }
	pub fn base_sum(self) -> Self { self.base_type(BaseType::Sum) }
	pub fn add_fields(mut self, mut fields: Vec<Field>) -> Self {
		self.fields.append(&mut fields);
		self
	}
	pub fn add_field(mut self, field: Field) -> Self {
		self.fields.push(field);
		self
	}
}

impl IdentBuilder {
	pub fn one_generic(mut self, generic: Typed<Ident>) -> Self {
		self.attributes.generics = GenericList(vec![generic]);
		self
	}
	pub fn name_str(mut self, name: &'static str) -> Self {
		self.name = name.into();
		self
	}
}

impl FieldBuilder {
	pub fn name_str(mut self, name: &'static str) -> Self {
		self.name = name.into();
		self
	}
}