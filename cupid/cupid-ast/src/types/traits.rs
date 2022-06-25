use crate::{expr::{function::Function, ident::Ident}, attr::Attr};

#[derive(Debug, Default, Clone)]
pub struct Trait {
    pub name: Ident,
    pub methods: Vec<(Ident, Function)>,
    pub attr: Attr,
}