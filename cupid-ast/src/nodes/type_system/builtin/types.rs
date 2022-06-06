use crate::*;

lazy_static! {
	pub static ref TYPE: Type<'static> = primitive("type");
	pub static ref TRAIT: Type<'static> = primitive("trait");
	pub static ref NOTHING: Type<'static> = primitive("nothing");
}

pub fn primitive(name: &'static str) -> Type {
	TypeBuilder::primitive(name)
}
