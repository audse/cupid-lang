use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Return<'src> {
        pub value: Option<EntryId>,
    }
}

impl<'src> From<Return<'src>> for Expr<'src> {
    fn from(value: Return<'src>) -> Self {
        Expr::Return(value.into())
    }
}

pub struct ReturnSource<'src> {
    pub return_kw: Token<'src>,
    pub value: Option<SourceId>,
}
