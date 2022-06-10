use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub ImplementBuilder => pub Implement {
		pub for_type: Ident,

		#[tabled(display_with = "fmt_option")]
		pub for_trait: Option<Ident>,

		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method>,

		pub attributes: Attributes
	}
}

impl UseAttributes for Implement {
	fn attributes(&self) -> &Attributes {
		&self.attributes
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		&mut self.attributes
	}
}

impl UseClosure for Implement {}