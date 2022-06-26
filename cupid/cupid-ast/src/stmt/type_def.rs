use crate::{attr::Attr, expr::ident::Ident, types::typ::Type};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub TypeDefBuilder => pub TypeDef {
        pub ident: Ident,
        pub value: Type,
        pub attr: Attr,
    }
}