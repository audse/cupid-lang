use crate::attr::{Attr, GetAttr};

use super::{ident::Ident, Expr};


cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub FunctionCallBuilder => pub FunctionCall {
        pub function: Ident,
        pub args: Vec<Expr>,
        pub attr: Attr,
    }
}

impl GetAttr for FunctionCall {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}