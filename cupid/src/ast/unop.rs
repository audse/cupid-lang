use super::{Expr, ExprHeader, Header};
use crate::{
    arena::EntryId,
    token::{Token, TokenType},
    with_header,
};

with_header! {
    #[derive(Debug, Clone)]
    pub struct UnOp<'src> {
        pub expr: EntryId,
        pub op: TokenType,
    }
}

impl<'src> From<UnOp<'src>> for Expr<'src> {
    fn from(value: UnOp<'src>) -> Self {
        Expr::UnOp(value.into())
    }
}

pub struct UnOpSource<'src> {
    pub op: Token<'src>,
    pub expr: EntryId,
}
