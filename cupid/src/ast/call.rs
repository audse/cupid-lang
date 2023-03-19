use super::{Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Call<'src> {
        pub callee: EntryId,
        pub args: Vec<EntryId>,
    }
}

pub struct CallSource<'src> {
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
    pub callee_src: SourceId,
    pub args_src: Vec<SourceId>,
    pub commas: Vec<Token<'src>>,
}

impl<'src> From<Call<'src>> for Expr<'src> {
    fn from(value: Call<'src>) -> Self {
        Expr::Call(value)
    }
}
