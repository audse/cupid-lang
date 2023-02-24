use crate::{
    attr::{Attr, GetAttr},
    stmt::allocate::Allocate,
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeDef(pub Allocate);

impl GetAttr for TypeDef {
    fn attr(&self) -> Attr {
        self.0.attr
    }
}
