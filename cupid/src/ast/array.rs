use crate::{arena::EntryId, token::Token, with_header};

use super::{Expr, ExprHeader, Header, SourceId};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Array<'src> {
        pub items: Vec<EntryId>,
    }
}

#[derive(Debug, Clone)]
pub struct ArraySource<'src> {
    pub open_bracket: Token<'src>,
    pub close_bracket: Token<'src>,
    pub commas: Vec<Token<'src>>,
    pub items: Vec<SourceId>,
}

impl<'src> From<Array<'src>> for Expr<'src> {
    fn from(value: Array<'src>) -> Self {
        Expr::Array(value.into())
    }
}
