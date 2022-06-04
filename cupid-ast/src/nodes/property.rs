use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub PropertyBuilder => pub Property {
		pub object: Box<Typed<Exp>>,
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

impl From<PropertyTerm> for Exp {
	fn from(property: PropertyTerm) -> Self {
		use PropertyTerm::*;
		match property {
			FunctionCall(function_call) => Exp::FunctionCall(function_call),
			Index(i, attr) => Exp::Value(Value {
				val: Untyped(Val::Integer(i as i32)),
				attributes: attr
			}),
			Term(exp) => *exp
		}
	}
}