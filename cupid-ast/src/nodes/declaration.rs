use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub DeclarationBuilder => pub Declaration {
		pub type_hint: Typed<Ident>,
		pub name: Ident,
		pub value: Box<Typed<Exp>>,
		
        #[tabled(skip)]
		pub mutable: bool,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}

impl UseAttributes for Declaration {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}