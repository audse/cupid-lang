use crate::{attr::Attr, expr::ident::Ident, types::traits::Trait};

#[derive(Debug, Default, Clone)]
pub struct TraitDef {
    pub ident: Ident,
    pub value: Trait,
    pub attr: Attr,
}