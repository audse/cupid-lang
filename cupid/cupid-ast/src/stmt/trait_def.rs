use crate::{
    attr::{Attr, GetAttr},
    stmt::allocate::Allocate,
};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitDef(pub Allocate);

impl GetAttr for TraitDef {
    fn attr(&self) -> Attr {
        self.0.attr
    }
}
