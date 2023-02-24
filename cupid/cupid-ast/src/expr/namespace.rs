use crate::{
    attr::{Attr, GetAttr},
    expr::Expr,
};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub NamespaceBuilder => pub Namespace {
        pub namespace: Box<Expr>,
        pub value: Box<Expr>,
        pub attr: Attr,
    }
}

impl GetAttr for Namespace {
    fn attr(&self) -> Attr {
        self.attr
    }
}
