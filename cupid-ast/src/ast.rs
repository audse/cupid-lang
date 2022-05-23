use crate::*;

pub trait TypeOf {
	fn type_of(&self, _scope: &mut Env) -> Type { NOTHING.to_owned() }
}


pub trait Analyze {
	fn resolve_names(&mut self, _scope: &mut Env) -> Result<(), Error> { Ok(()) }
	fn resolve_types(&mut self, _scope: &mut Env) -> Result<(), Error> { Ok(()) }
	fn check_types(&mut self, _scope: &mut Env) -> Result<(), Error> { Ok(()) }
}

pub trait Interpret {
	fn interpret<T>(&mut self, scope: &mut Env) -> Result<T, Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Typed<T: Default> {
	Untyped(T),
	Typed(T, Type),
}
impl<T: Default> Default for Typed<T> {
	fn default() -> Self {
    	Self::Untyped(T::default())
	}
}

impl<T: Default> Typed<T> {
	pub fn inner(&self) -> &T {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
	pub fn get_type(&self) -> &Type {
		if let Self::Typed(_, t) = self {
			t
		} else {
			panic!("no type found")
		}
	}
	pub fn into_typed(self, type_val: Type) -> Self {
		if let Self::Untyped(t) = self {
			Self::Typed(t, type_val)
		} else {
			self
		}
	}
	pub fn to_typed(&mut self, type_val: Type) {
		if let Self::Untyped(t) = self {
			*self = Self::Typed(std::mem::take(t), type_val);
		}
	}
}

impl Typed<Ident> {
	pub fn nothing() -> Self {
		let nothing = NOTHING.to_owned();
		let nothing_ident = nothing.into_ident();
		Self::Typed(nothing_ident, nothing)
	}
}

impl<T: Default> std::ops::Deref for Typed<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
}