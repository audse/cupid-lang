use crate::{attr::Attr, expr::ident::Ident, types::traits::Trait};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub TraitDefBuilder => pub TraitDef {
        pub ident: Ident,
        pub value: Trait,
        pub attr: Attr,
    }
}