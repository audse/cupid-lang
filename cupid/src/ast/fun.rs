use super::{Define, Expr, ExprHeader, Header, SourceId};
use crate::{arena::EntryId, compiler::FunctionType, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Fun<'src> {
        pub kind: FunctionType,
        pub name: Option<Token<'src>>,
        pub params: Vec<Define<'src>>,
        pub body: EntryId,
    }
}

pub struct FunSource<'src> {
    pub fun_kw: Token<'src>,
    pub name: Option<Token<'src>>,
    pub params: SourceId,
    pub body: SourceId,
}

impl<'src> From<Fun<'src>> for Expr<'src> {
    fn from(value: Fun<'src>) -> Self {
        Expr::Fun(value)
    }
}
