use crate::{expr::{ident::Ident, Expr}, attr::Attr};


#[derive(Debug, Default, Clone)]
pub struct Decl {
    pub ident: Ident,
    pub type_annotation: Option<Ident>,
    pub value: Expr,
    pub attr: Attr,
}