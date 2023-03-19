use std::fmt;

use super::{ExprHeader, Fun, Header};
use crate::with_header;

with_header! {
    #[derive(Clone)]
    pub struct Method<'src> {
        pub name: &'src str,
        pub fun: Fun<'src>,
    }
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
