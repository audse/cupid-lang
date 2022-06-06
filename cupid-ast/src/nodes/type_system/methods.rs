use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
	pub MethodBuilder => pub Method<'ast> {
		pub name: Ident,
		pub value: Function<'ast>,
	}
}