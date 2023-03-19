use super::{Expr, ExprHeader, Header};
use crate::{arena::EntryId, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Call<'src> {
        pub callee: EntryId,
        pub args: Vec<EntryId>,
    }
}

impl<'src> From<Call<'src>> for Expr<'src> {
    fn from(value: Call<'src>) -> Self {
        Expr::Call(value)
    }
}
