use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Tabled)]
pub enum Typed<T: Default> {
	Untyped(T),
	Typed(T, Type),
}

impl<T: Default> Default for Typed<T> {
	fn default() -> Self {
		Self::Untyped(T::default())
	}
}

impl<T: Default + std::fmt::Debug> Typed<T> {
	pub fn inner(&self) -> &T {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
	pub fn inner_mut(&mut self) -> &mut T {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
	pub fn into_inner(self) -> T {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
	pub fn get_type(&self) -> Result<&Type, ErrCode> {
		if let Self::Typed(_, t) = self {
			Ok(t)
		} else {
			Err(ERR_EXPECTED_TYPE)
		}
	}
	pub fn get_node_type(&self) -> Result<&Type, ASTErr> where T: UseAttributes {
		if let Self::Typed(_, t) = self {
			Ok(t)
		} else {
			Err((self.source(), ERR_EXPECTED_TYPE))
		}
	}
	pub fn get_type_mut(&mut self) -> &mut Type {
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
	pub fn into_map<R: Default>(self, closure: &dyn Fn(T) -> R) -> Typed<R> {
		match self {
			Self::Untyped(t) => Typed::Untyped(closure(t)),
			Self::Typed(t, mut type_val) => Typed::Typed(closure(t), std::mem::take(&mut type_val))
		}
	}
}

impl Typed<Ident> {
	pub fn nothing() -> Self {
		let nothing = NOTHING.to_owned();
		let nothing_ident = nothing.to_owned().into_ident();
		Self::Typed(nothing_ident, nothing)
	}
}

impl TypeOf for Typed<Ident> {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
		match self {
			Self::Typed(_, t) => Ok(t.to_owned()),
			Self::Untyped(v) => v.type_of(scope)
		}
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

impl<T: Default> std::ops::DerefMut for Typed<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Untyped(t) => t,
			Self::Typed(t, _) => t
		}
	}
}

impl <T: Default + std::fmt::Display> std::fmt::Display for Typed<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Typed(v, t) => write!(f, "typed [{v}]\n{t}"),
			Self::Untyped(v) => write!(f, "untyped [{v}]"),
		}
	}
}