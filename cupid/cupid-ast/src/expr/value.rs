use derive_more::{From, TryInto};
use crate::{attr::Attr, types::{typ::Type, traits::Trait}};

#[derive(Debug, Default, Clone)]
pub struct Value {
	pub inner: Val,
	pub attr: Attr
}

#[derive(Debug, Default, Clone, From, TryInto)]
pub enum Val {
	VBoolean(bool),
	VChar(char),
	VDecimal(i32, u32),
	VInteger(i32),
	VString(cupid_util::Str),
	VType(Type),
	VTrait(Trait),
	#[default]
	VNone,
}

impl From<Type> for Value {
	fn from(t: Type) -> Self {
		Self {
			attr: t.attr,
			inner: Val::VType(t)
		}
	}
}

impl From<Trait> for Value {
	fn from(t: Trait) -> Self {
		Self {
			attr: t.attr,
			inner: Val::VTrait(t)
		}
	}
}