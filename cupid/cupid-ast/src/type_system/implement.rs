use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub ImplementBuilder => pub Implement {
		pub for_type: Ident,
		pub for_trait: Option<Ident>,
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