use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub FunctionCallBuilder => pub FunctionCall {
		pub function: Typed<(Ident, Option<Function>)>,
		pub args: Vec<Typed<Exp>>,
		pub attributes: Attributes
	}
}

impl UseAttributes for FunctionCall {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}