use super::{Expr, ExprHeader, Header, SourceId};
use crate::{value::Value, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Constant<'src> {
        pub value: Value,
    }
}

pub struct ConstantSource {
    pub value_src: SourceId,
}

impl<'src> From<Constant<'src>> for Expr<'src> {
    fn from(value: Constant<'src>) -> Self {
        Expr::Constant(value)
    }
}
