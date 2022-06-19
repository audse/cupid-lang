use cupid_util::{Bx, InvertOption, Str};
// use IsTyped::*;

crate::util::node_builder! {
	#[derive(Debug, Default, Clone)]
	pub IdentBuilder => pub Ident {
		pub generics: Vec<Ident>,
		pub name: Str,
		pub namespace: Option<Box<Ident>>,
	}
}

impl Ident {
	pub fn new<C: Into<Str>>(name: C, attr: crate::Attributes) -> Self {
		Self { name: name.into(), attr, ..Default::default() }
	}
}

impl std::hash::Hash for Ident {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for Ident {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name 
			&& self.generics.len() == other.generics.len()
	}
}

impl Eq for Ident {}

impl Ident {
    pub fn pass(
		self, 
		generic_fun: impl FnOnce(Vec<Self>, &mut crate::Env) -> crate::PassResult<Vec<Self>>, 
		fun: impl FnOnce(Self, &mut crate::Env) -> crate::PassResult<Self>,
		env: &mut crate::Env
	) -> crate::PassResult<Self> {
        let Self { generics, name, namespace, attr } = self;
        Ok(Ident::build()
			.generics(generic_fun(generics, env)?)
			.name(name)
			.namespace(namespace.map(|n| Ok(fun(*n, env)?.bx())).invert()?)
            .attr(attr)
            .build())
    }
}