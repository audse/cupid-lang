use crate::*;

build_struct! {
	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Tabled)]
	pub ImplementBuilder => pub Implement<'ast> {
		pub for_type: Ident,

		#[tabled(display_with = "fmt_option")]
		pub for_trait: Option<Ident>,

		#[tabled(display_with = "fmt_vec")]
		pub methods: Vec<Method<'ast>>,

		pub attributes: Attributes
	}
}