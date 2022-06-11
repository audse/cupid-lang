use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default)]
	pub TraitBuilder => pub Trait {
		pub name: Ident,
		pub methods: Vec<Method>,
		pub bounds: Vec<Ident>,
	}
}

impl Trait {
	pub fn into_ident(&self) -> Ident {
		self.name.to_owned()
	}
}

impl ToIdent for Trait { 
	fn to_ident(&self) -> Ident { self.name.to_owned() } 
}

impl PartialEq for Trait { 
	fn eq(&self, other: &Self) -> bool { self.name == other.name } 
}

impl Eq for Trait {}

impl Hash for Trait {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl From<Trait> for Value {
	fn from(t: Trait) -> Self {
		VTrait(t)
	}
}

impl UseAttributes for Trait {
	fn attributes(&self) -> &Attributes {
		&self.name.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.name.attributes
	}
}