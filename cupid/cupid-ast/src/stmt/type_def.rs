use crate::{attr::Attr, expr::ident::Ident, types::typ::Type};

#[derive(Debug, Default, Clone)]
pub struct TypeDef {
    pub ident: Ident,
    pub value: Type,
    pub attr: Attr,
}