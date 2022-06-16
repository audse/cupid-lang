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
	pub fn inner_mut(&mut self) -> &mut T {
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
		generic_fun: impl FnOnce(Vec<IsTyped<Self>>, &mut crate::Env) -> crate::PassResult<Vec<IsTyped<Self>>>, 
		fun: impl FnOnce(Self, &mut crate::Env) -> crate::PassResult<Self>,
		env: &mut crate::Env
	) -> crate::PassResult<Self> {
        let Self { generics, name, namespace, address, attr } = self;
        Ok(Ident::build()
			.generics(generic_fun(generics, env)?)
			.name(name)
			.namespace(namespace.map(|n| Ok(fun(*n, env)?.bx())).invert()?)
			.address(address)
            .attr(attr)
            .build())
    }
}

impl IsTyped<Ident> {
	pub fn pass(
		self, 
		generic_fun: impl FnOnce(Vec<IsTyped<Ident>>, &mut crate::Env) -> crate::PassResult<Vec<IsTyped<Ident>>>,
		fun: impl FnOnce(Ident, &mut crate::Env) -> crate::PassResult<Ident>,
		env: &mut crate::Env
	) -> crate::PassResult<Self> {
		match self {
			Untyped(ident) => Ok(Untyped(ident.pass(generic_fun, fun, env)?)),
			Typed(ident, t) => Ok(Typed(ident.pass(generic_fun, fun, env)?, t)),
		}
	}
}

impl crate::AsNode for IsTyped<Ident> {
	fn source(&self) -> usize { self.inner().attr.source }
	fn scope(&self) -> usize { self.inner().attr.scope }
	fn typ(&self) -> usize { self.inner().attr.typ }
	fn set_source(&mut self, source: usize) { self.inner_mut().attr.source = source; }
	fn set_scope(&mut self, scope: usize) { self.inner_mut().attr.scope = scope; }
	fn set_typ(&mut self, typ: usize) { self.inner_mut().attr.typ = typ; }
}