use crate::{attr::Attr, expr::block::Block, stmt::decl::Decl};

#[derive(Debug, Default, Clone)]
pub struct Function {
    pub params: Vec<Decl>,
    pub body: Block,
    pub attr: Attr,
}