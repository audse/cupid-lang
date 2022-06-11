use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub PropertyBuilder => pub Property {
		pub object: Box<Typed<Exp>>,
		pub property: Typed<PropertyTerm>,
	}
}

impl UseAttributes for Property {
    fn attributes(&self) -> &Attributes {
        self.object.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        self.object.attributes_mut()
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
			Index(i, attr) => Exp::Value(VInteger(i as i32, attr)),
			Term(exp) => *exp
		}
	}
}

impl UseAttributes for PropertyTerm {
	fn attributes(&self) -> &Attributes {
		match self {
			Self::FunctionCall(function_call) => function_call.attributes(),
			Self::Index(_, attr) => attr,
			Self::Term(term) => term.attributes()
		}
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		match self {
			Self::FunctionCall(function_call) => function_call.attributes_mut(),
			Self::Index(_, attr) => attr,
			Self::Term(term) => term.attributes_mut()
		}
	}
}