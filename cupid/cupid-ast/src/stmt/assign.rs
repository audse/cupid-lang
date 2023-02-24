use crate::{
    attr::{Attr, GetAttr},
    stmt::allocate::Allocate,
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Assign(pub Allocate);

impl GetAttr for Assign {
    fn attr(&self) -> Attr {
        self.0.attr
    }
}
