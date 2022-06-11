use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub DeclarationBuilder => pub Declaration {
		pub type_hint: Typed<Ident>,
		pub name: Ident,
		pub value: Box<Typed<Exp>>,
		pub mutable: bool,
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