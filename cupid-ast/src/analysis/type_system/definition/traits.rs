use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, Tabled)]
	pub TraitBuilder => pub Trait {
		pub name: Ident,
		
		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method>,

		#[tabled(display_with = "fmt_vec")]
		pub bounds: Vec<Ident>,
	}
}

impl Trait {
	pub fn into_ident(&self) -> Ident {
		self.name.to_owned()
	}
}

impl UseAttributes for Trait {
	fn attributes(&mut self) -> &mut Attributes { self.name.attributes() }
}

impl Analyze for Trait {} // TODO

impl ToIdent for Trait { 
	fn to_ident(&self) -> Ident { self.name.to_owned() } 
}

impl From<Trait> for Val { 
	fn from(t: Trait) -> Val { Val::Trait(t) } 
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
