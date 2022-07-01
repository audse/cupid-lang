use crate::{attr::{Attr, GetAttr}, expr::Expr};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub BlockBuilder => pub Block {
        pub expressions: Vec<Expr>,
        pub attr: Attr,
    }
}

impl GetAttr for Block {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}