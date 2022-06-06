use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub FunctionBuilder => pub Function<'ast> {
		pub body: Typed<Block<'ast>>,
		
		#[tabled(display_with = "fmt_vec")]
		pub params: Vec<Declaration<'ast>>,

		pub return_type: Typed<Ident>,

		pub attributes: Attributes,
	}
}
