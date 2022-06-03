use crate::*;

lazy_static! {
	pub static ref TYPE: Type = primitive("type");
	pub static ref TRAIT: Type = primitive("trait");
	pub static ref NOTHING: Type = primitive("nothing");
}

pub fn primitive(name: &'static str) -> Type {
	TypeBuilder::primitive(name)
}
