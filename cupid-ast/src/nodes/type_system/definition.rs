use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled, Default)]
	pub TypeDefinitionBuilder => pub TypeDefinition {
		pub name: Ident,
		pub fields: FieldSet,
		pub base_type: BaseType,
	}
}

impl From<TypeDefinition> for Type {
	fn from(def: TypeDefinition) -> Self {
		Type::build()
			.name(def.name)
			.fields(def.fields)
			.base_type(def.base_type)
			.build()
	}
}