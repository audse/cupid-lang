use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, token::TokenType, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct BinOp<'src> {
        pub left: EntryId,
        pub right: EntryId,
        pub op: TokenType,
    }
}

impl<'src> From<BinOp<'src>> for Expr<'src> {
    fn from(value: BinOp<'src>) -> Self {
        Expr::BinOp(value)
    }
}
