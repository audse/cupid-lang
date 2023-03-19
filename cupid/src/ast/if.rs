use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

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
