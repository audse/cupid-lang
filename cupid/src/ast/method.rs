use std::fmt;

use super::{ExprHeader, Fun, FunSource, Header};
use crate::{token::Token, with_header};

with_header! {
    #[derive(Clone)]
    pub struct Method<'src> {
        pub name: Token<'src>,
        pub fun: Fun<'src>,
    }
}

pub struct MethodSource<'src> {
    pub name: Token<'src>,
    pub fun: FunSource<'src>,
}

impl fmt::Debug for Method<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name)
            .field("params", &self.fun.params)
            .field("body", &self.fun.body)
            .finish()
    }
}
