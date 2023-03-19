use super::{Expr, ExprHeader, Header, SourceId};
use crate::{
    arena::EntryId,
    token::{Token, TokenType},
    with_header,
};

with_header! {
    #[derive(Debug, Clone)]
    pub struct BinOp<'src> {
        pub left: EntryId,
        pub right: EntryId,
        pub op: TokenType,
    }
}

pub struct BinOpSource<'src> {
    pub left_src: SourceId,
    pub right_src: SourceId,
    pub op: Token<'src>,
}

impl<'src> From<BinOp<'src>> for Expr<'src> {
    fn from(value: BinOp<'src>) -> Self {
        Expr::BinOp(value)
    }
}
