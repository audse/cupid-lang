use crate::{expr::ident::Ident, attr::{Attr, GetAttr}, stmt::decl::Decl};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
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
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}