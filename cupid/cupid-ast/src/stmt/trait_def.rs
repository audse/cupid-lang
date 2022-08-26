use crate::{
    attr::{Attr, GetAttr},
    expr::ident::Ident,
    types::traits::Trait,
};
use std::{cell::RefCell, rc::Rc};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub TraitDefBuilder => pub TraitDef {
        pub ident: Ident,
        pub value: Rc<RefCell<Trait>>,
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
