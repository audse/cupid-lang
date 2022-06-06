use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub PropertyBuilder => pub Property<'ast> {
		pub object: Box<Typed<Exp<'ast>>>,
		pub property: Typed<PropertyTerm<'ast>>,

        #[tabled(skip)]
		pub attributes: Attributes,
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled)]
pub enum PropertyTerm<'ast> {
	FunctionCall(Box<FunctionCall<'ast>>),
	Index(usize, Attributes),
	Term(Box<Exp<'ast>>),
}

impl Default for PropertyTerm<'_> {
	fn default() -> Self { Self::Term(Box::new(Exp::Empty)) }
}

impl From<PropertyTerm<'_>> for Exp<'_> {
	fn from(property: PropertyTerm) -> Self {
		use PropertyTerm::*;
		match property {
			FunctionCall(function_call) => Exp::FunctionCall(function_call),
			Index(i, attr) => Exp::Value(Box::new(Value {
				value: Untyped(i as i32),
				attributes: attr
			})),
			Term(exp) => *exp
		}
	}
}