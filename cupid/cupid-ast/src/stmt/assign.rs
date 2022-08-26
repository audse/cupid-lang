use crate::{
    attr::{Attr, GetAttr},
    expr::{ident::Ident, Expr},
};
use std::{cell::RefCell, rc::Rc};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub AssignBuilder => pub Assign {
        pub ident: Ident,
        pub value: Rc<RefCell<Expr>>,
        pub attr: Attr,
    }
}

impl GetAttr for Assign {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}
