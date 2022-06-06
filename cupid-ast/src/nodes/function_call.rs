use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub FunctionCallBuilder => pub FunctionCall<'ast> {

		#[tabled(display_with = "fmt_function")]
		pub function: Typed<(Ident, Option<Function<'ast>>)>,

		#[tabled(display_with = "fmt_vec")]
		pub args: Vec<Typed<Exp<'ast>>>,
		
        #[tabled(skip)]
		pub attributes: Attributes,
	}
}

fn fmt_function(function: &Typed<(Ident, Option<Function>)>) -> String {
	format!("{} {}", function.0, fmt_option!(&function.1))
}