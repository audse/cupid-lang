use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub MethodBuilder => pub Method {
		pub name: Ident,
		pub value: Function,
	}
}

impl UseAttributes for Method {
	fn attributes(&self) -> &Attributes { 
		self.name.attributes() 
	}
	fn attributes_mut(&mut self) -> &mut Attributes { 
		self.name.attributes_mut() 
	}
}

impl UseClosure for Method {}