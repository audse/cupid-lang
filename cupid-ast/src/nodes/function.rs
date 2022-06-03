use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub FunctionBuilder => pub Function {
		pub body: Typed<Block>,
		
		#[tabled(display_with = "fmt_vec")]
		pub params: Vec<Declaration>,

		pub return_type: Typed<Ident>,

		pub attributes: Attributes,
	}
}
