use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
	pub FieldBuilder => pub Field {
		pub name: Ident,
		pub type_hint: Option<Typed<Ident>>
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FieldSet(pub Vec<Field>);

impl FieldSet {
	pub fn find(&self, name: &Ident) -> Option<&Field> {
		self.iter().find(|f| &f.name == name)
	}
}

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
			.type_hint(f.type_hint.unwrap_or_else(|| IsTyped(Type::type_ty().into_ident(), Type::type_ty())))
			.name(f.name)
			.attributes(attr)
			.build()
	}
}

impl UseAttributes for Field {
	fn attributes(&self) -> &Attributes {
		self.name.attributes()
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		self.name.attributes_mut()
	}
}

impl UseAttributes for FieldSet {
	fn attributes(&self) -> &Attributes {
		unreachable!("cannot get attributes of field set")
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		unreachable!("cannot get attributes of field set")
	}
}