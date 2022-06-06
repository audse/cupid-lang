use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled, Default)]
	pub TypeDefBuilder => pub TypeDef<'ast> {
		pub name: Ident,
		pub fields: FieldSet,
		pub base_type: BaseType,
	}
}

impl From<TypeDef<'_>> for Type<'_> {
	fn from(def: TypeDef) -> Self {
		Type::build()
			.name(def.name)
			.fields(def.fields)
			.base_type(def.base_type)
			.build()
	}
}