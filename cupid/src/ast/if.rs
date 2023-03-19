use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct If<'src> {
        pub condition: EntryId,
        pub body: EntryId,
        pub else_body: Option<EntryId>,
    }
}

impl<'src> From<If<'src>> for Expr<'src> {
    fn from(value: If<'src>) -> Self {
        Expr::If(value.into())
    }
}

pub struct IfSource<'src> {
    pub if_kw: Token<'src>,
    pub condtion: SourceId,
    pub body: SourceId,
    pub else_kw: Option<Token<'src>>,
    pub else_body: Option<SourceId>,
}
