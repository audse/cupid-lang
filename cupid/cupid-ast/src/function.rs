use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub FunctionBuilder => pub Function {
		pub body: Typed<Block>,
		pub params: Vec<Declaration>,
		pub return_type: Typed<Ident>,
		pub attributes: Attributes,
	}
}

impl UseAttributes for Function {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}