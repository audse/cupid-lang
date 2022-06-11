use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub TypeDefBuilder => pub TypeDef {
		pub name: Ident,
		pub fields: FieldSet,
		pub base_type: BaseType,
	}
}

impl From<TypeDef> for Type {
	fn from(def: TypeDef) -> Self {
		Type::build()
			.name(def.name)
			.fields(def.fields)
			.base_type(def.base_type)
			.build()
	}
}

impl UseAttributes for TypeDef {
	fn attributes(&self) -> &Attributes {
		&self.name.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.name.attributes
	}
}