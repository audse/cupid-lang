use crate::{
    attr::{Attr, GetAttr},
    expr::{ident::Ident, Expr},
    types::{traits::Trait, typ::Type},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AllocationStage {
    TypeNameResolution,
    #[default]
    NameResolution,
    Runtime,
}

#[derive(
    Debug,
    Clone,
    derive_more::From,
    derive_more::TryInto,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Unwrap,
)]
pub enum Allocation {
    Type(Rc<RefCell<Type>>),
    Trait(Rc<RefCell<Trait>>),
    Expr(Rc<RefCell<Expr>>),
}

impl Default for Allocation {
    fn default() -> Self {
        Self::Expr(Rc::new(RefCell::new(Expr::default())))
    }
}

impl GetAttr for Allocation {
    fn attr(&self) -> Attr {
        match self {
            Self::Expr(x) => x.borrow().attr(),
            Self::Trait(x) => x.borrow().attr(),
            Self::Type(x) => x.borrow().attr(),
        }
    }
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub AllocateBuilder => pub Allocate {
        pub ident: Ident,
        pub value: Allocation,
        pub stage: AllocationStage,
        pub attr: Attr,
    }
}

impl GetAttr for Allocate {
    fn attr(&self) -> Attr {
        self.attr
    }
}

impl Allocation {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Expr(e) => e.borrow().is_empty(),
            _ => false,
        }
    }
}
