use crate::{arena::EntryId, with_header};

use super::{Expr, ExprHeader, Header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Array<'src> {
        pub items: Vec<EntryId>,
    }
}

impl<'src> From<Array<'src>> for Expr<'src> {
    fn from(value: Array<'src>) -> Self {
        Expr::Array(value.into())
    }
}
