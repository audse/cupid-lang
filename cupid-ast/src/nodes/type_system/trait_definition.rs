use crate::*;

build_struct! {
	#[derive(Debug, PartialEq, Eq, Hash, Clone, Default, Tabled)]
	pub TraitDefBuilder => pub TraitDef<'ast> {
		pub name: Ident,

		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method<'ast>>,

		#[tabled(display_with = "fmt_vec")]
		pub bounds: Vec<Ident>,

		pub attributes: Attributes,
	}
}

impl From<TraitDef<'_>> for Trait<'_> {
	fn from(def: TraitDef) -> Self {
		Trait::build()
			.name(def.name)
			.methods(def.methods)
			.bounds(def.bounds)
			.build()
	}
}