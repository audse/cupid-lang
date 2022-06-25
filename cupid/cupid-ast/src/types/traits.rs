use crate::{expr::ident::Ident, attr::Attr, stmt::decl::Decl};

#[derive(Debug, Default, Clone)]
pub struct Trait {
    pub ident: Ident,
    pub methods: Vec<Decl>,
    pub attr: Attr,
}