use crate::{
    attr::{Attr, GetAttr},
    expr::ident::Ident,
    stmt::decl::Decl,
};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TraitBuilder => pub Trait {
        pub ident: Ident,
        pub methods: Vec<Decl>,
        pub attr: Attr,
    }
}

impl GetAttr for Trait {
    fn attr(&self) -> Attr {
        self.attr
    }
}
