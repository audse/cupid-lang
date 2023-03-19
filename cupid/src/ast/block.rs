use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Block<'src> {
        pub body: Vec<EntryId>,
    }
}

impl<'src> From<Block<'src>> for Expr<'src> {
    fn from(value: Block<'src>) -> Self {
        Expr::Block(value.into())
    }
}
