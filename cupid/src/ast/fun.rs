use super::{Define, Expr, ExprHeader, Header};
use crate::{arena::EntryId, compiler::FunctionType, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Fun<'src> {
        pub kind: FunctionType,
        pub name: Option<&'src str>,
        pub params: Vec<Define<'src>>,
        pub body: EntryId,
    }
}

impl<'src> From<Fun<'src>> for Expr<'src> {
    fn from(value: Fun<'src>) -> Self {
        Expr::Fun(value)
    }
}
