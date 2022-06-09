use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub FieldBuilder => pub Field {
		pub name: Ident,
		#[tabled(display_with="fmt_option")]
		pub type_hint: Option<Typed<Ident>>
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct FieldSet (
	#[tabled(display_with="fmt_vec")]
	pub Vec<Field>
);

impl std::ops::Deref for FieldSet {
	type Target = Vec<Field>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for FieldSet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<&Declaration> for Field {
	fn from(dec: &Declaration) -> Self {
		Field {
			name: dec.name.to_owned(),
			type_hint: Some(dec.type_hint.to_owned())
		}
	}
}

impl From<Field> for Declaration {
	fn from(f: Field) -> Self {
		let attr = f.name.attributes.to_owned();
		Declaration::build()
			.type_hint(f.type_hint.unwrap_or_else(|| IsTyped(type_type().into_ident(), type_type())))
			.name(f.name)
			.attributes(attr)
			.build()
	}
}