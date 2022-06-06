use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, Tabled)]
	pub TraitBuilder => pub Trait<'ast> {
		pub name: Ident,
		
		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method<'ast>>,

		#[tabled(display_with = "fmt_vec")]
		pub bounds: Vec<Ident>,
	}
}

impl Trait<'_> {
	pub fn into_ident(&self) -> Ident {
		self.name.to_owned()
	}
}

impl ToIdent for Trait<'_> { 
	fn to_ident(&self) -> Ident { self.name.to_owned() } 
}

impl PartialEq for Trait<'_> { 
	fn eq(&self, other: &Self) -> bool { self.name == other.name } 
}

impl Eq for Trait<'_> {}

impl Hash for Trait<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl From<Trait<'_>> for Value<Trait<'_>> {
	fn from(t: Trait) -> Self {
		Value::build()
			.attributes(t.attributes().to_owned())
			.val(IsTyped(t, TRAIT.to_owned()))
			.build()
	}
}