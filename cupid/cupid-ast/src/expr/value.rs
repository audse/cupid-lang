use crate::attr::{Attr, GetAttr};
use derive_more::{From, TryInto};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Value {
    pub inner: Val,
    pub attr: Attr,
}

#[derive(Debug, Default, Clone, From, TryInto, serde::Serialize, serde::Deserialize)]
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
}
