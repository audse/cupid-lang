use crate::*;

build_struct! {
	#[derive(Debug, PartialEq, Eq, Hash, Clone, Default, Tabled)]
	pub TraitDefBuilder => pub TraitDef {
		pub name: Ident,

		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method>,

		#[tabled(display_with = "fmt_vec")]
		pub bounds: Vec<Ident>,
	}
}

impl From<TraitDef> for Trait {
	fn from(def: TraitDef) -> Self {
		Trait::build()
			.name(def.name)
			.methods(def.methods)
			.bounds(def.bounds)
			.build()
	}
}

impl UseAttributes for TraitDef {
	fn attributes(&self) -> &Attributes {
		&self.name.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.name.attributes
	}
}

impl UseClosure for TraitDef {}