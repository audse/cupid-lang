use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub DeclarationBuilder => pub Declaration<'ast> {
		pub type_hint: Typed<Ident>,
		pub name: Ident,
		pub value: Box<Typed<Exp<'ast>>>,
		
        #[tabled(skip)]
		pub mutable: bool,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}