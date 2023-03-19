use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Define<'src> {
        pub name: &'src str,
        pub value: Option<EntryId>,
    }
}

impl<'src> From<Define<'src>> for Expr<'src> {
    fn from(value: Define<'src>) -> Self {
        Expr::Define(value)
    }
}
