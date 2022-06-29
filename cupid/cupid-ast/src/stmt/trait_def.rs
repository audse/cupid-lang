use crate::{attr::{Attr, GetAttr}, expr::ident::Ident, types::traits::Trait};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub TraitDefBuilder => pub TraitDef {
        pub ident: Ident,
        pub value: Trait,
        pub attr: Attr,
    }
}

impl GetAttr for TraitDef {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}