use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Break<'src> {
        pub value: Option<EntryId>,
    }
}

impl<'src> From<Break<'src>> for Expr<'src> {
    fn from(value: Break<'src>) -> Self {
        Expr::Break(value.into())
    }
}
