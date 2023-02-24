use crate::{
    attr::{Attr, GetAttr},
    expr::ident::Ident,
    stmt::allocate::Allocate,
};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub DeclBuilder => pub Decl {
        pub allocate: Allocate,
        pub mutable: Mut,
        pub type_annotation: Option<Ident>,
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
        self.allocate.attr
    }
}
