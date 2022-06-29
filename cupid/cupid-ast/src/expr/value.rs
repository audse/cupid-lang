use derive_more::{From, TryInto};
use crate::attr::{Attr, GetAttr};

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
	#[default]
	VNone,
}

impl GetAttr for Value {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}