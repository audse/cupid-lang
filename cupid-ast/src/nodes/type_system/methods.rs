use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub MethodBuilder => pub Method {
		pub name: Ident,
		pub signature: Type,
		#[tabled(display_with = "fmt_option")]
		pub value: Option<Function>,
	}
}