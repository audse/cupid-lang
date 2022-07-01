use crate::{attr::{Attr, GetAttr}, expr::ident::Ident, types::typ::Type};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TypeDefBuilder => pub TypeDef {
        pub ident: Ident,
        pub value: Type,
        pub attr: Attr,
    }
}

impl GetAttr for TypeDef {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}