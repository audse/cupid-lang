use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Loop<'src> {
        pub body: EntryId,
    }
}

impl<'src> From<Loop<'src>> for Expr<'src> {
    fn from(value: Loop<'src>) -> Self {
        Expr::Loop(value.into())
    }
}

pub struct LoopSource<'src> {
    pub loop_kw: Token<'src>,
    pub body: SourceId,
}
