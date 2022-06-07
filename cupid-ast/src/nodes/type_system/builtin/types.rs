use crate::*;

pub fn type_type() -> Type {
	primitive("type")
}

pub fn trait_type() -> Type {
	primitive("trait")
}

pub fn nothing_type() -> Type {
	primitive("nothing")
}

pub fn primitive(name: &'static str) -> Type {
	TypeBuilder::primitive(name)
}
