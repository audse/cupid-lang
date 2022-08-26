use crate::{
    attr::{Attr, GetAttr},
    expr::{ident::Ident, Expr},
};
use std::{cell::RefCell, rc::Rc};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub DeclBuilder => pub Decl {
        pub ident: Ident,
        pub mutable: Mut,
        pub type_annotation: Option<Ident>,
        pub value: Rc<RefCell<Expr>>,
        pub attr: Attr,
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Mut {
    Mutable,
    #[default]
    Immutable,
}

impl GetAttr for Decl {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}
