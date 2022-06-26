use crate::{expr::ident::Ident, attr::Attr, stmt::decl::Decl};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub TraitBuilder => pub Trait {
        pub ident: Ident,
        pub methods: Vec<Decl>,
        pub attr: Attr,
    }
}