use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Define<'src> {
        pub name: Token<'src>,
        pub value: Option<EntryId>,
    }
}

impl<'src> From<Define<'src>> for Expr<'src> {
    fn from(value: Define<'src>) -> Self {
        Expr::Define(value)
    }
}

pub struct DefineSource<'src> {
    pub let_kw: Option<Token<'src>>,
    pub equal: Option<Token<'src>>,
    pub name: Token<'src>,
    pub value: Option<SourceId>,
}
