use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericList(
	pub Vec<Typed<Ident>>
);

impl From<Vec<&'static str>> for GenericList {
	fn from(names: Vec<&'static str>) -> Self {
    	Self(names.into_iter().map(|n| Untyped(Ident::new_name(n)) ).collect::<Vec<Typed<Ident>>>())
	}
}

impl std::ops::Deref for GenericList {
	type Target = Vec<Typed<Ident>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for GenericList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl UseAttributes for GenericList {
	fn attributes(&self) -> &Attributes {
		unreachable!("cannot get attributes of generic list")
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		unreachable!("cannot get attributes of generic list")
	}
}