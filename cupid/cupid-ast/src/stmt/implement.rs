use crate::{
    attr::{Attr, GetAttr},
    expr::ident::Ident,
    stmt::{allocate::Allocate, decl::Decl},
};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub ImplBuilder => pub Impl {
        pub trait_ident: Ident,
        pub type_ident: Ident,
        pub methods: Vec<Decl>,
        pub attr: Attr
    }
}

impl GetAttr for Impl {
    fn attr(&self) -> Attr {
        self.attr
    }
}
