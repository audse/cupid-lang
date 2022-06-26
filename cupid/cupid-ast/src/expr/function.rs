use crate::{attr::Attr, expr::{block::Block, ident::Ident}, stmt::decl::Decl};

#[derive(Debug, Default, Clone)]
pub struct Function {
    pub params: Vec<Decl>,
    pub body: Block,
    pub return_type_annotation: Option<Ident>,
    pub attr: Attr,
}