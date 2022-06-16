use cupid_util::{Bx, InvertOption};
use IsTyped::*;

cupid_util::node_builder! {
	#[derive(Debug, Default, Clone)]
	pub IdentBuilder => pub Ident {
		pub generics: Vec<IsTyped<Ident>>,
		pub name: cupid_util::Str,
		pub namespace: Option<Box<Ident>>,
		pub address: Option<crate::Address>
	}
}

#[derive(Debug, Clone)]
pub enum IsTyped<T: std::fmt::Debug + Default + Clone> {
	Typed(T, crate::Address),
	Untyped(T)
}

impl<T: std::fmt::Debug + Default + Clone> Default for IsTyped<T> {
	fn default() -> Self {
		Self::Untyped(T::default())
	}
}

impl<T: std::fmt::Debug + Default + Clone> IsTyped<T> {
	pub fn into_inner(self) -> T {
		match self {
			Typed(inner, _) | Untyped(inner) => inner
		}
	}
	pub fn inner(&self) -> &T {
		match self {
			Typed(inner, _) | Untyped(inner) => inner
		}
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
		generic_fun: impl FnOnce(Vec<IsTyped<Ident>>, &mut crate::Env) -> crate::PassResult<Vec<IsTyped<Ident>>>, 
		fun: impl FnOnce(Self, &mut crate::Env) -> crate::PassResult<Ident>, 
		env: &mut crate::Env
	) -> crate::PassResult<Ident> {
        let Ident { generics, name, namespace, address, attr } = self;
        Ok(Ident::build()
			.generics(generic_fun(generics, env)?)
			.name(name)
			.namespace(namespace.map(|n| Ok(fun(*n, env)?.bx())).invert()?)
			.address(address)
            .attr(attr)
            .build())
    }
}