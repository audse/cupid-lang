use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub PropertyBuilder => pub Property {
		pub object: Typed<Box<Exp>>,
		pub property: Typed<PropertyTerm>,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled)]
pub enum PropertyTerm {
	FunctionCall(Box<FunctionCall>),
	Index(usize, Attributes),
	Term(Box<Exp>),
}

impl Default for PropertyTerm {
	fn default() -> Self { Self::Term(Box::new(Exp::Empty)) }
}