use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Break<'src> {
        pub value: Option<EntryId>,
    }
}

pub struct BreakSource<'src> {
    pub break_kw: Token<'src>,
    pub value_src: Option<SourceId>,
}

impl<'src> From<Break<'src>> for Expr<'src> {
    fn from(value: Break<'src>) -> Self {
        Expr::Break(value.into())
    }
}
