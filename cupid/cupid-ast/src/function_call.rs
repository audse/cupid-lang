use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub FunctionCallBuilder => pub FunctionCall {

		#[tabled(display_with = "fmt_function")]
		pub function: Typed<(Ident, Option<Function>)>,

		#[tabled(display_with = "fmt_vec")]
		pub args: Vec<Typed<Exp>>,

		#[tabled(skip)]
		pub attributes: Attributes
	}
}

fn fmt_function(function: &Typed<(Ident, Option<Function>)>) -> String {
	format!("{} {}", function.0, fmt_option!(&function.1))
}

impl UseAttributes for FunctionCall {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl UseClosure for FunctionCall {}
