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
	pub fn generic_arg(mut self, index: usize, generic: Typed<Ident>) -> Self {
		self.name.attributes.generics.0[index] = generic;
		self
	}
	pub fn generics(mut self, generics: GenericList) -> Self {
		self.name.attributes.generics = generics;
		self
	}
	pub fn generics_str(mut self, generics: Vec<&'static str>) -> Self {
		self.name.attributes.generics = GenericList::from(generics);
		self
	}
	pub fn named_fields(mut self, fields: Vec<(Str, Typed<Ident>)>) -> Self {
		self.fields = FieldSet::Named(fields);
		self
	}
	pub fn unnamed_fields(mut self, fields: Vec<Typed<Ident>>) -> Self {
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
		self.generics(GenericList::from(vec![generic, generic, generic]))
			.unnamed_fields(vec![
				Untyped(Ident::new_name(generic)),
				Untyped(Ident::new_name(generic)),
				Untyped(Ident::new_name(generic)),
			])
			.base_type(BaseType::Function)
	}
	pub fn base_primitive(mut self, name: &'static str) -> Self {
		self.base_type = BaseType::Primitive(name.into());
		self
	}
}


impl ValueBuilder {
	pub fn typed_val(mut self, val: Val, val_type: Type) -> Self {
		self.val = IsTyped(val, val_type);
		self
	}
	pub fn untyped_val<V: Into<Val>>(mut self, val: V) -> Self {
		self.val = Untyped(val.into());
		self
	}
	pub fn none(mut self) -> Self {
		self.val = IsTyped(Val::None, NOTHING.to_owned());
		self
	}
	pub fn builtin(mut self) -> Self {
		self.val = IsTyped(Val::BuiltinPlaceholder, NOTHING.to_owned());
		self
	}
}