use crate::{attr::Attr, expr::Expr};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub BlockBuilder => pub Block {
        pub expressions: Vec<Expr>,
        pub attr: Attr,
    }
}