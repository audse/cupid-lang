use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub DeclarationBuilder => pub Declaration {
		pub type_hint: Typed<Ident>,
		pub name: Ident,
		pub value: Typed<Box<Exp>>,
		
        #[tabled(skip)]
		pub mutable: bool,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}