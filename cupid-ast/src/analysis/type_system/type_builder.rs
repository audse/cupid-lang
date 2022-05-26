use crate::*;

impl TypeBuilder {
	pub fn name_str(mut self, name: &'static str) -> Self {
		self.name.name = name.into();
		self
	}
	pub fn add_generic(mut self, index: usize, generic: GenericParam) -> Self {
		self.name.attributes.generics.0.insert(index, generic);
		self
	}
	pub fn generic_arg(mut self, index: usize, generic: Ident) -> Self {
		self.name.attributes.generics.0[index].1 = Some(generic);
		self
	}
	pub fn generics(mut self, generics: GenericParams) -> Self {
		self.name.attributes.generics = generics;
		self
	}
	pub fn generics_str(mut self, generics: Vec<&'static str>) -> Self {
		self.name.attributes.generics = GenericParams::from(generics);
		self
	}
	pub fn named_fields(mut self, fields: Vec<TypedIdent>) -> Self {
		self.fields = FieldSet::Named(fields);
		self
	}
	pub fn unnamed_fields(mut self, fields: Vec<Ident>) -> Self {
		self.fields = FieldSet::Unnamed(fields);
		self
	}
	pub fn primitive(name: &'static str) -> Type {
		Self::new()
			.name_str(name)
			.base_type(BaseType::Primitive(name.into()))
			.build()
	}
	pub fn bin_op(self, generic: &'static str) -> Self {
		self.generics(GenericParams::from(vec![generic, generic, generic]))
			.unnamed_fields(vec![
				Ident::new_name(generic),
				Ident::new_name(generic),
				Ident::new_name(generic),
			])
			.base_type(BaseType::Function)
	}
}