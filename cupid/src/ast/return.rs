use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Return<'src> {
        pub value: Option<EntryId>,
    }
}

impl<'src> From<Return<'src>> for Expr<'src> {
    fn from(value: Return<'src>) -> Self {
        Expr::Return(value.into())
    }
}
