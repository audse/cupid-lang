use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

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
